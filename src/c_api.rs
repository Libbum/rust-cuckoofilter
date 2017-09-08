use CuckooFilter;
use CuckooError;
use std::collections::hash_map::DefaultHasher;

#[allow(non_camel_case_types)]
pub type rcf_cuckoofilter = CuckooFilter<DefaultHasher>;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum rcf_cuckoofilter_status {
  Ok,
  NotFound,
  NotEnoughSpace,
  NotSupported,
}
// #[repr(C)]
// const STATUS_DESCRIPTIONS: &'static ['static str] = &["Ok", "Not found", "Not enough Space", "Not supported"];

// #[no_mangle]
// pub extern fn rcf_cuckoofilter_status_desc(rcf_cuckoofilter_status status) -> {
//   return STATUS_DESCRIPTIONS[status as u8]
// }


#[no_mangle]
pub extern fn rcf_cuckoofilter_with_capacity(capacity: usize) -> *mut rcf_cuckoofilter {
    let cf = CuckooFilter::with_capacity(capacity);
    let cf = Box::new(cf);
    Box::into_raw(cf)
}

#[no_mangle]
pub extern fn rcf_cuckoofilter_free(cf: *mut rcf_cuckoofilter) {
    let cf = unsafe { Box::from_raw(cf) };
    drop(cf);
}

#[no_mangle]
pub extern fn rcf_cuckoofilter_contains(cf: *const rcf_cuckoofilter, data: u64) -> rcf_cuckoofilter_status {
    let cf = unsafe { cf.as_ref() };
    cf.map_or(rcf_cuckoofilter_status::NotSupported,
              |cf| if cf.contains(&data) { rcf_cuckoofilter_status::Ok } else { rcf_cuckoofilter_status::NotFound })
}

#[no_mangle]
pub extern fn rcf_cuckoofilter_add(cf: *mut rcf_cuckoofilter, data: u64) -> rcf_cuckoofilter_status {
    let cf = unsafe { cf.as_mut() };
    cf.map_or(rcf_cuckoofilter_status::NotSupported,
              |cf| match cf.add(&data) {
                Ok(_) => rcf_cuckoofilter_status::Ok,
                Err(CuckooError::NotEnoughSpace) => rcf_cuckoofilter_status::NotEnoughSpace
              })
}

#[no_mangle]
pub extern fn rcf_cuckoofilter_len(cf: *const rcf_cuckoofilter) -> usize {
    let cf = unsafe { cf.as_ref() };
    cf.map_or(0, |cf| cf.len())
}

#[no_mangle]
pub extern fn rcf_cuckoofilter_is_empty(cf: *const rcf_cuckoofilter) -> bool {
    let cf = unsafe { cf.as_ref() };
    cf.map_or(true, |cf| cf.is_empty())
}

#[no_mangle]
pub extern fn rcf_cuckoofilter_memory_usage(cf: *const rcf_cuckoofilter) -> usize {
    let cf = unsafe { cf.as_ref() };
    cf.map_or(0, |cf| cf.memory_usage())
}


#[no_mangle]
pub extern fn rcf_cuckoofilter_delete(cf: *mut rcf_cuckoofilter, data: u64) -> rcf_cuckoofilter_status {
    let cf = unsafe { cf.as_mut() };
    cf.map_or(rcf_cuckoofilter_status::NotSupported,
              |cf| if cf.delete(&data) { rcf_cuckoofilter_status::Ok } else { rcf_cuckoofilter_status::NotFound })
}
