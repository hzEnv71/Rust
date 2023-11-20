pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // 断言 assert
    // assert!  assert_eq!   assert_ne! 可以添加自定义信息
    #[test]
    // #[ignore]
    // 忽略测试
    // #[ignore]  添加字段就会忽略
    // cargo test -- --ignored 运行有忽略属性的测试
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4,"执行失败");
        // assert_ne!(result,4,"执行失败");
    }
    #[test]
    // #[should_panic]
    #[should_panic(expected = "this is a panic")]//expected检查 panic 发生时输出的错误提示信息是否包含了指定的文字
    fn should_panic(){
        panic!("this is a panic");
    }

    //Result<T,E> 来编写测试，它运行失败时会返回一个 Err 值而不panic
    #[test]
    fn test1()->Result<(),String>{
        assert_eq!(add(1,1),2);
        Ok(())
    }

}
