pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn median(mut list: Vec<f32>) -> Option<f32>{
    if list.is_empty(){
        return None
    }

    list.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let num_items = list.len();
    let middle = num_items / 2;

    let median = if num_items % 2 == 0{
        (list[middle -1] + list[middle + 1]) / 2.0
    } else {
        list[middle]
    };

    Some(median)

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn ut_median_from_odd_length_list(){
        let list = vec![1.0, 6.0, 8.0, 3.0, 7.0];
        assert_eq!(Some(6.0), median(list))
    }

    #[test]
    fn ut_median_from_even_length_list(){
        let list = vec![1.0, 6.0, 8.0, 3.0, 7.0, 2.0];
        assert_eq!(Some(5.0), median(list))
    }

    #[test]
    fn ut_median_none_from_empty_list(){
        let list = vec![];
        assert_eq!(None, median(list))
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


