use std::convert::From;
use std::ffi::{c_char, c_double, c_int, c_short, c_void};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NgComplex {
    pub cx_real: c_double,
    pub cx_imag: c_double
}

pub use num_complex::Complex64;
impl From<NgComplex> for Complex64 {
    fn from(ng: NgComplex) -> Self {
        unsafe { std::mem::transmute(ng) }
    }
}


#[derive(Clone, Debug)]
#[repr(C)]
pub struct NgVectorInfo {
    v_name: *const c_char, // Constrain all T* data from ngspice to be immutable
    v_type: c_int,
    v_flags: c_short,
    v_realdata: *const c_double,
    v_compdata: *const NgComplex,
    v_length: c_int
}

#[derive(Copy, Clone, Debug)]
pub enum VectorType {
    NOTYPE         = 0,
    TIME           = 1,
    FREQUENCY      = 2,
    VOLTAGE        = 3,
    CURRENT        = 4,
    OUTPUT_N_DENS  = 5,
    OUTPUT_NOISE   = 6,
    INPUT_N_DENS   = 7,
    INPUT_NOISE    = 8,
    POLE           = 9,
    ZERO           = 10,
    SPARAM         = 11,
    TEMP           = 12,
    RES            = 13,
    IMPEDANCE      = 14,
    ADMITTANCE     = 15,
    POWER          = 16,
    PHASE          = 17,
    DB             = 18,
    CAPACITANCE    = 19,
    CHARGE         = 20
}

#[derive(Copy, Clone, Debug)]
pub enum VectorFlag {
    REAL = 1 << 0,
    COMPLEX = 1 << 1,
    ACCUM = 1 << 2,
    PLOT = 1 << 3,
    PRINT = 1 << 4,
    MINGIVEN = 1 << 5,
    MAXGIVEN = 1 << 6,
    PERMANENT = 1 << 7,
}

impl VectorFlag {
    pub fn flag(flag: Vec<VectorFlag>) -> i16 {
        flag.iter().fold(0, |acc, f| acc | *f as i16)
    }
}


pub struct VectorInfo {
    name: String,
    vtype: i32,
    flags: i16,
    realdata: Option<Vec<f64>>,
    compdata: Option<Vec<Complex64>>,
    length: i32
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NgVecValues {
    name: *const c_char,
    creal: c_double,
    cimag: c_double,
    is_scale: bool,
    is_complex: bool
}

pub struct VecValues {
    pub name: String,
    pub creal: f64,
    pub cimag: f64,
    pub is_scale: bool,
    pub is_complex: bool
}

impl VecValues {
    pub unsafe fn from(ng: &NgVecValues) -> VecValues {
        VecValues {
            name: std::ffi::CStr::from_ptr(ng.name).to_str().unwrap().to_string(),
            creal: ng.creal,
            cimag: ng.cimag,
            is_scale: ng.is_scale,
            is_complex: ng.is_complex
        }
    }
}



#[derive(Clone, Debug)]
#[repr(C)]
pub struct NgVecValuesAll {
    pub veccount: c_int,
    pub vecindex: c_int,
    pub vecsa: *const *const NgVecValues
}

pub struct VecValuesAll {
    pub count: i32,
    pub index: i32,
    pub vecsa: Vec<VecValues>
}

impl VecValuesAll {
    pub unsafe fn from(ng: &NgVecValuesAll) -> VecValuesAll {
        unsafe {
            let slice = std::slice::from_raw_parts(ng.vecsa, ng.veccount as usize);
            let mut vals = Vec::<VecValues>::with_capacity(ng.veccount as usize);
            slice.iter().for_each(|x| {
                vals.push(VecValues::from(&**x));
            });

            VecValuesAll {
                count: ng.veccount,
                index: ng.vecindex,
                vecsa: vals
            }
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NgVecInfo {
    number: c_int,
    vecname: *const c_char,
    is_real: bool,
    pdvec: *const c_void, // TODO: what is dvec *d and dvec *ds
    pdvecscale: *const c_void
}

pub struct VecInfo {
    pub number: i32,
    pub name: String,
    pub is_real: bool,
}

impl VecInfo {
    pub unsafe fn from(ng: &NgVecInfo) -> VecInfo {
        VecInfo {
            number: ng.number,
            name: std::ffi::CStr::from_ptr(ng.vecname).to_str().unwrap().to_string(),
            is_real: ng.is_real
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NgVecInfoAll {
    name: *const c_char,
    title: *const c_char,
    date: *const c_char,
    type_: *const c_char,
    veccount: c_int,
    vecs: *const *const NgVecInfo
}
pub struct VecInfoAll {
    pub name: String,
    pub title: String,
    pub date: String,
    pub type_: String,
    pub count: i32,
    pub vecs: Vec<VecInfo>
}

impl VecInfoAll {
    pub unsafe fn from(ng: &NgVecInfoAll) -> VecInfoAll {
        VecInfoAll {
            name: std::ffi::CStr::from_ptr(ng.name).to_str().unwrap().to_string(),
            title: std::ffi::CStr::from_ptr(ng.title).to_str().unwrap().to_string(),
            date: std::ffi::CStr::from_ptr(ng.date).to_str().unwrap().to_string(),
            type_: std::ffi::CStr::from_ptr(ng.type_).to_str().unwrap().to_string(),
            count: ng.veccount,
            vecs: std::slice::from_raw_parts(ng.vecs, ng.veccount as usize).iter().map(|x| VecInfo::from(&**x)).collect()
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NgEvtData {
    dcop: c_int,
    step: c_double,
    node_value: *const c_char
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NgEvtSharedData {
    evt_dect: *const *const NgEvtData,
    num_steps: c_int
}
