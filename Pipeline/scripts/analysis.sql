-- select all projects that have both rust and cxx IR
SELECT * FROM abicheck_issues WHERE has_rust_ir=true and has_cxx_ir=true;