use memscan::search::MemorySearch;
use nix::libc::{iovec, process_vm_readv};

use crate::{
    app_error::AppError,
    data_types::{DataType, WrappedValue},
    endian::Endianness,
    process_picker::Process,
    search_scope,
    settings::Settings,
};

pub struct SearchRegion {
    pub pid: i32,
    pub start: u64,
    pub end: u64,
}

impl SearchRegion {
    pub fn load(processes: &[Process], search_scope: search_scope::SearchScope) -> Vec<Self> {
        let mut regions = vec![];
        for process in processes {
            let proc = procfs::process::Process::new(process.pid).unwrap();
            let maps = proc.maps().unwrap();
            for map in maps {
                if search_scope.is_in_scope(&map) {
                    regions.push(Self {
                        pid: process.pid,
                        start: map.address.0,
                        end: map.address.1,
                    });
                }
            }
        }
        regions
    }
}

pub fn search_sync(
    regions: &[SearchRegion],
    data_type: DataType,
    input: &str,
    settings: &Settings,
    endianness: Endianness,
) -> Result<Vec<(i32, Vec<u64>)>, AppError> {
    let mut results = vec![];

    let step_size: usize = settings.search_buffer_size;
    let mut read_buffer = vec![0; step_size];
    let wrapped_value = data_type.parse(input)?;

    for region in regions {
        let mut pointers = vec![];
        for chunk_start in (region.start..region.end).step_by(step_size) {
            let chunk_end = (chunk_start + step_size as u64).min(region.end);
            let chunk_length = (chunk_end - chunk_start) as usize;
            println!("Reading from {:X} to {:X}", chunk_start, chunk_end);

            let local_iov = [iovec {
                iov_base: read_buffer.as_mut_ptr() as *mut _,
                iov_len: read_buffer.len(),
            }];
            let remote_iov = [iovec {
                iov_base: chunk_start as *mut _,
                iov_len: (chunk_length) as usize,
            }];

            let read_size =
                unsafe { process_vm_readv(region.pid, &local_iov as _, 1, &remote_iov as _, 1, 0) };

            if read_size < 0 {
                // return Err(AppError::from_errno());
                let error = AppError::from_errno();
                println!(
                    "[ERROR]: Failed to read memory for PID {}: {}",
                    region.pid, error
                );
                continue;
            }
    
            if read_size != chunk_length as isize {
                println!(
                    "[WARN]: Failed to read entire memory: {} vs. {}",
                    read_size, chunk_length
                );
            }

            for offset in wrapped_value.scan_memory(&read_buffer, endianness) {
                pointers.push(chunk_start + offset as u64);
            }
        }
        if !pointers.is_empty() {
            results.push((region.pid, pointers));
        }
    }

    Ok(results)
}

pub fn search_continue_sync(
    results: &[(i32, Vec<u64>)],
    data_type: DataType,
    input: &str,
    settings: &Settings,
    endianness: Endianness,
) -> Result<Vec<(i32, Vec<u64>)>, AppError> {
    let mut new_results = vec![];

    let size = data_type.size();
    let mut read_buffer = vec![0u8; size];
    let wrapped_value = data_type.parse(input)?;

    for (pid, pointers) in results.iter() {
        let mut new_pointers = vec![];

        for pointer in pointers {
            let local_iov = [iovec {
                iov_base: read_buffer.as_mut_ptr() as *mut _,
                iov_len: read_buffer.len(),
            }];
            let remote_iov = [iovec {
                iov_base: *pointer as *mut _,
                iov_len: size as usize,
            }];

            let read_size =
                unsafe { process_vm_readv(*pid, &local_iov as _, 1, &remote_iov as _, 1, 0) };

            if read_size < 0 {
                // return Err(AppError::from_errno());
                let error = AppError::from_errno();
                println!(
                    "[ERROR]: Failed to read memory for PID {}: {}",
                    pid, error
                );
                continue;
            }

            if read_size != size as isize {
                println!(
                    "[WARN]: Failed to read entire memory: {} vs. {}",
                    read_size, size
                );
            }

            let matches = wrapped_value.compare_to(&read_buffer, endianness);

            if matches {
                new_pointers.push(*pointer);
            }
        }

        if !new_pointers.is_empty() {
            new_results.push((*pid, new_pointers));
        }
    }

    Ok(new_results)
}

pub fn read_value(pid: i32, pointer: u64, data_type: DataType) -> Result<WrappedValue, AppError> {
    let size = data_type.size();
    let mut read_buffer = vec![0u8; size];

    let local_iov = [iovec {
        iov_base: read_buffer.as_mut_ptr() as *mut _,
        iov_len: read_buffer.len(),
    }];
    let remove_iov = [iovec {
        iov_base: pointer as *mut _,
        iov_len: size as usize,
    }];

    let read_size = unsafe { process_vm_readv(pid, &local_iov as _, 1, &remove_iov as _, 1, 0) };

    if read_size < 0 {
        return Err(AppError::from_errno());
    }

    if read_size != size as isize {
        println!(
            "[WARN]: Failed to read entire memory: {} vs. {}",
            read_size, size
        );
    }

    let wrapped_value = data_type.cast(&read_buffer, Endianness::Native)?;

    Ok(wrapped_value)
}
