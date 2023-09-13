use std::{
    ffi::{c_char, CStr},
    fs::File,
    io::{BufReader, BufWriter},
    ops::DerefMut,
    path::Path,
    sync::RwLock,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref SAVED_TENSORS: RwLock<AllSavedTensors> = RwLock::new(Default::default());
    static ref SAVED_INPUTS_RECORD: RwLock<InputRecord> = RwLock::new(Default::default());
    pub static ref CURRENT_INPUT_TOKEN: RwLock<String> = RwLock::new(Default::default());
}

pub fn save_input() {
    let saved_tensors = std::mem::replace(
        SAVED_TENSORS.write().unwrap().deref_mut(),
        Default::default(),
    );
    let current_input_token = std::mem::replace(
        CURRENT_INPUT_TOKEN.write().unwrap().deref_mut(),
        Default::default(),
    );
    SAVED_INPUTS_RECORD
        .write()
        .unwrap()
        .records
        .push((current_input_token, saved_tensors));
}

#[no_mangle]
/// a test function,should print "Hello, world!"
pub extern "C" fn rust_utils_hello() {
    println!("Hello, world!");
}

#[no_mangle]
/// clear the saved tensors
pub extern "C" fn rust_utils_clear() {
    let mut value = SAVED_TENSORS.write().unwrap();
    value.tensors.clear();
}

#[no_mangle]
/// print the saved tensors
pub extern "C" fn rust_utils_print() {
    let value = SAVED_TENSORS.read().unwrap();
    println!("{:?}", &value);
}

pub fn print_tensor(data: &[f32], ne: &[i64; 4], nb: &[u64; 4]) {
    for n3 in 0..ne[3] {
        for n2 in 0..ne[2] {
            for n1 in 0..ne[1] {
                print!("({:?}-{:?}-{:?}): ", n3, n2, n1);
                for n0 in 0..ne[0] {
                    let index_byte =
                        n3 as u64 * nb[3] + n2 as u64 * nb[2] + n1 as u64 * nb[1] + n0 as u64 * 4;
                    print!("{:?} ", data[(index_byte / 4) as usize]);
                }
                println!();
            }
            println!("------{}-{}------", n3, n2);
        }
    }
}

pub fn print_shapes(ne: &[i64; 4], nb: &[u64; 4]) {
    println!("shape:{:?} stride:{:?}", ne, nb);
}
/// test if the tensor has no padding, return true if no padding
pub fn no_padding(ne: &[i64; 4], nb: &[usize; 4]) -> bool {
    let no_padding_nb1 = ne[0] as usize * nb[0];
    nb[1] == no_padding_nb1
}

#[no_mangle]
/// add a tensor to the saved tensors, the data are ggml format
/// - `name` is the name of the tensor, should be valid c/c++ string
/// - `data` is the data of the tensor, should be a pointer to a f32 array in ggml format
pub extern "C" fn rust_utils_add_element(
    name: *const c_char,
    data: *const f32,
    ne: &[i64; 4],
    nb: &[u64; 4],
) {
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_str().unwrap();
    let data = unsafe { std::slice::from_raw_parts(data, (nb[3] * ne[3] as u64) as usize) };
    let data = Tensor {
        inner: data.to_vec(),
        dim: ne.clone(),
        stride: nb.clone(),
    };

    let mut value = SAVED_TENSORS.write().unwrap();
    value.tensors.push((name.to_string(), data));
}

#[no_mangle]
/// save the saved tensors to a file
pub extern "C" fn rust_utils_save_elements(path: *const c_char) {
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_str().unwrap();
    // save the result using bincode
    let value = SAVED_INPUTS_RECORD.read().unwrap();
    value.save_to_file(Path::new(path));
}

#[no_mangle]
/// load from a file
pub extern "C" fn rust_utils_load_elements(path: *const c_char) {
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_str().unwrap();
    let value = AllSavedTensors::load_from_file(Path::new(path));
    let mut saved_tensors = SAVED_TENSORS.write().unwrap();
    *saved_tensors = value;
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct AllSavedTensors {
    pub tensors: Vec<(String, Tensor)>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct InputRecord {
    /// the input token, and the saved tensors
    pub records: Vec<(String, AllSavedTensors)>,
}

impl InputRecord {
    pub fn save_to_file(&self, file: &Path) {
        bincode::serialize_into(BufWriter::new(File::create(file).unwrap()), self).unwrap();
    }
}
/// a ggm tensor
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Tensor {
    /// the data itself
    pub inner: Vec<f32>,
    /// the shape of the tensor
    pub dim: [i64; 4],
    /// the stride of each dimention in byte, `stride[0]` is always `4`, `stride[1] = dim[0]*stride[0]+padding` , `stride[n] = dim[n-1]*stride[n-1] for n>1`
    pub stride: [u64; 4],
}
pub fn rust_utils_get() -> AllSavedTensors {
    SAVED_TENSORS.read().unwrap().clone()
}

impl AllSavedTensors {
    pub fn load_from_file(file: &Path) -> Self {
        bincode::deserialize_from(BufReader::new(File::open(file).unwrap())).unwrap()
    }

    pub fn save_to_file(&self, file: &Path) {
        bincode::serialize_into(BufWriter::new(File::create(file).unwrap()), self).unwrap();
    }

    pub fn print_all_shapes(&self) {
        for i in self.tensors.iter() {
            println!("name:{}", i.0);
            let Tensor {
                inner: _,
                dim,
                stride,
            } = &i.1;
            print_shapes(dim, stride);
        }
    }

    pub fn print_all_tensor(&self) {
        for i in self.tensors.iter() {
            println!("name:{}", i.0);
            let Tensor { inner, dim, stride } = &i.1;
            print_tensor(inner, dim, stride);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_print() {
        let data = [1., 2., 3., 4.];
        let ne = [2, 2, 1, 1];
        let nb = [4, 8, 16, 16];
        super::print_tensor(&data, &ne, &nb);
    }
    #[test]
    fn test_print_gap() {
        let data = [1., 2., 0., 0., 3., 4., 0., 0.];
        let ne = [2, 2, 1, 1];
        let nb = [4, 16, 32, 32];
        super::print_tensor(&data, &ne, &nb);
    }
}
