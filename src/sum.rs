pub fn sum(list:&[u32])->Option<u32>{
    let mut result:Option<u32>=Some(0);
    let size = list.len();
    for i in 0..size{
        result=result.unwrap().checked_add(list[i]);
    }
    return result;
}

#[test]
fn test_empty_list(){
    let empty_list=[];
    let result=sum(&empty_list);
    assert_eq!(result, Some(0));
}


#[test]
fn test_list(){
    let list=[1,2,3,4,5];
    let result=sum(&list);
    assert_eq!(result, Some(15));
}

#[test]
fn test_overflow_list(){
    let overflow_list=[u32::MAX,1];
    let result=sum(&overflow_list);
    assert_eq!(result, None);
}