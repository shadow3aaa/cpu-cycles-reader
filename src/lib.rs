//! 这只为读取CpuCycles特化，不是对[perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)的完整封装
//! ⚠ 权限要求: 确保程序拥有root权限

mod ffi;

use std::{collections::HashMap, ptr, slice};

use ffi::CyclesReaderRaw;
use libc::{c_int, c_longlong as c_ll, c_void};

pub struct CyclesReader {
    raw_ptr: *mut CyclesReaderRaw,
    cpus: Vec<c_int>,
}

impl Drop for CyclesReader {
    fn drop(&mut self) {
        unsafe { ffi::destroyCyclesReader(self.raw_ptr) } // ffi里面已经drop了指针，不需要ptr::drop_in_place
        self.raw_ptr = ptr::null_mut();
    }
}

impl CyclesReader {
    /// 创建CyclesReader
    /// ```ignore
    /// use cpu_cycles_reader::CyclesReader;
    /// let reader = CyclesReader::new(&[0, 1, 2, 3]);
    /// ```
    pub fn new(cpus: &[c_int]) -> Result<Self, &'static str> {
        let cpus = cpus.to_vec();
        let cpus_ptr = cpus.as_ptr();

        let raw_ptr = unsafe { ffi::createCyclesReader(cpus_ptr, cpus.len()) };
        if raw_ptr.is_null() {
            return Err("Failed to create CyclesReader");
        }
        Ok(Self { raw_ptr, cpus })
    }

    /// 开启Cycles监视
    /// ```ignore
    /// use cpu_cycles_reader::CyclesReader;
    /// let reader = CyclesReader::new(&[0, 1, 2, 3]).unwrap();
    /// reader.enable();
    /// ```
    pub fn enable(&self) {
        unsafe {
            ffi::enableCyclesReader(self.raw_ptr);
        }
    }

    /// 关闭Cycles监视
    /// ```ignore
    /// use cpu_cycles_reader::CyclesReader;
    /// let reader = CyclesReader::new(&[0, 1, 2, 3]).unwrap();
    /// reader.disable();
    /// ```
    pub fn disable(&self) {
        unsafe {
            ffi::disableCyclesReader(self.raw_ptr);
        }
    }

    /// 读取从开启到现在的Cycles数
    /// 按照构造函数cpu参数顺序返回
    pub fn read(&self) -> Result<HashMap<c_int, c_ll>, &'static str> {
        let raw = unsafe { ffi::readCyclesReader(self.raw_ptr) };

        if raw.is_null() {
            return Err("CyclesReader failed to read");
        }

        let slice = unsafe { slice::from_raw_parts(raw, (*self.raw_ptr).size) };
        let map = self.cpus.iter().zip(slice).map(|(c, d)| (*c, *d)).collect(); // copied here

        // 释放ffi malloc的数组
        unsafe { libc::free(raw as *mut c_void) }

        Ok(map)
    }
}
