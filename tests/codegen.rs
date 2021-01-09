use vicis::{
    codegen::{
        lower::convert_module,
        target::x86_64::{calling_conv::SystemV, X86_64},
    },
    // exec::{generic_value::GenericValue, interpreter::Interpreter},
    ir::module,
};

#[test]
fn gen() {
    //     let asm = r#"
    // source_filename = "asm.c"
    // target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    // target triple = "x86_64-pc-linux-gnu"
    //
    // ; Function Attrs: noinline nounwind optnone uwtable
    // define dso_local i32 @main() #0 {
    //   %a = alloca i32, align 4
    //   store i32 0, i32* %a
    //   ret i32 0
    // }
    //
    // attributes #0 = { noinline nounwind optnone uwtable }
    // "#;
    let asm = r#"
source_filename = "asm.c"                                                                          
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"     
target triple = "x86_64-pc-linux-gnu"                                                            

; Function Attrs: noinline nounwind optnone uwtable                                              
define dso_local i32 @main() #0 {                                                                
  %a = alloca i32, align 4
  store i32 2, i32* %a
  %b = load i32, i32* %a
  %c = add i32 %b, 1 ; 3
  %d = add i32 %b, 2 ; 4
  %e = add i32 %c, %d ; 7
  ret i32 %e
}                                                                                                

attributes #0 = { noinline nounwind optnone uwtable }
"#;
    let module = module::parse_assembly(asm).unwrap();
    // let main = module.find_function_by_name("main").unwrap();
    let mach_module = convert_module(X86_64::new(SystemV), module);
    println!("{}", mach_module);
    // let mut interpreter = Interpreter::new(&module);
    // assert_eq!(
    //     interpreter.run_function(main).unwrap(),
    //     GenericValue::Int32(0)
    // );
}