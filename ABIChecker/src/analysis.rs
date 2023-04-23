use std::collections::HashMap;

use collect_cxx::*;
use collect_rust::*;

use inkwell::{module::Module, targets::TargetData, types::StructType};

pub fn analysis_struct(rm: Module, cm: Module, rinfo: RInfo, cinfo: CInfo) {
    let mut rust_struct = HashMap::new();
    let mut cpp_struct = HashMap::new();

    for rs in rinfo.structs {
        let s = match rm.get_struct_type(&rs.name) {
            Some(s) => s,
            None => continue,
        };

        rust_struct.insert(rs.name, s);
    }

    for cs in cinfo.structs {
        let s = match cm.get_struct_type(&format!("class.{}", cs.name)) {
            Some(s) => s,
            None => match cm.get_struct_type(&format!("struct.{}", cs.name)) {
                Some(s) => s,
                None => continue,
            },
        };

        cpp_struct.insert(cs.name, s);
    }
}

pub fn struct_layout_analysis(a: StructType, b: StructType, target_data: TargetData) {
    let a_len = a.count_fields();
    let b_len = b.count_fields();

    let mut a_index = 0;
    let mut b_index = 0;

    let mut a_carry = 0;
    let mut b_carry = 0;

    loop {
        let a_field = a.get_field_type_at_index(a_index);
        let b_field = b.get_field_type_at_index(b_index);

        if let (Some(af), Some(bf)) = (a_field, b_field) {
            let a_size = target_data.get_store_size(&af);
            let b_size = target_data.get_store_size(&bf);

            println!("Debug:\n{:?}\n{:?}\n a_index:{}, b_index{}, a_carry{}, b_carry{}, a_size{}, bsize{}", af, bf, a_index, b_index, a_carry, b_carry, a_size, b_size);

            if a_carry > 0 {
                if a_size + a_carry == b_size {
                    a_index += 1;
                    b_index += 1;
                    a_carry = 0;
                } else if a_size + a_carry > b_size {
                    panic!("Mismatch")
                } else {
                    a_index += 1;
                    a_carry += a_size;
                }
            } else if b_carry > 0 {
                if b_size + b_carry == a_size {
                    a_index += 1;
                    b_index += 1;
                    b_carry = 0;
                } else if b_size + b_carry > a_size {
                    panic!("Mismatch")
                } else {
                    b_index += 1;
                    b_carry += b_size;
                }
            } else {
                if a_size == b_size {
                    a_index += 1;
                    b_index += 1;
                } else if a_size > b_size {
                    b_index += 1;
                    b_carry += b_size;
                } else {
                    a_index += 1;
                    a_carry += a_size;
                }
            }
        } else {
            // TODO
            break;
        }
    }
}
