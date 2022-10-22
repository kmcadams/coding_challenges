
//implement function to find any unique items in an iterable list and return them, preserving order
pub fn unique<T:PartialEq, I: IntoIterator<Item=T>>(list: I) -> Vec<T>{
    let mut encountered: Vec<T> = Vec::new();

    for item in list{
        if !encountered.contains(&item){
            encountered.push(item)
        }
    }
    encountered
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_empty_list() {
        let input_list: Vec<i32> = vec![];
        let expected_output = vec![];
        assert_eq!(unique(input_list), expected_output);
    }

    #[test]
    fn ut_sorted_list_with_duplicates() {
        let input_vec: Vec<i32> = vec![1,1,2,3,5,5];
        let expected_ouput = vec![1,2,3,5];        
        assert_eq!(unique(input_vec), expected_ouput);

        let input_vec: Vec<&str> = vec!["1","1","2","3","5","5"];
        let expected_output = vec!["1","2","3","5"];
        assert_eq!(unique(input_vec), expected_output);

        let input_arr = [1,1,2,3,5,5];
        let expected_ouput = vec![1,2,3,5];
        assert_eq!(unique(input_arr), expected_ouput);

    }

    #[test]
    fn ut_sorted_list_with_no_duplicates() {
        let input_list: Vec<i32> = vec![1,2,3,5];
        let expected_output = vec![1,2,3,5];
        assert_eq!(unique(input_list), expected_output);

        let input_list: Vec<&str> = vec!["1","2","3","5"];
        let expected_output = vec!["1","2","3","5"];
        assert_eq!(unique(input_list), expected_output);

        let input_arr = [1,2,3,5];
        let expected_ouput = vec![1,2,3,5];
        assert_eq!(unique(input_arr), expected_ouput);

    }

    #[test]
    fn ut_unsorted_list_with_duplicates() {
        let input_list: Vec<i32> = vec![1,4,4,2,3,1,5];
        let expected_output = vec![1,4,2,3,5];
        assert_eq!(unique(input_list), expected_output);

        let input_list: Vec<&str> = vec!["1","4","4","2","3","1","5"];
        let expected_output = vec!["1","4","2","3","5"];
        assert_eq!(unique(input_list), expected_output);

        let input_arr = [1,4,4,2,3,1,5];
        let expected_ouput = vec![1,4,2,3,5];
        assert_eq!(unique(input_arr), expected_ouput);

    }

    #[test]
    fn ut_unsorted_list_with_no_duplicates() {
        let input_list: Vec<i32> = vec![1,5,4,2,3];
        let expected_output = vec![1,5,4,2,3];
        assert_eq!(unique(input_list), expected_output);

        let input_list: Vec<&str> = vec!["1","5","4","2","3",];
        let expected_output = vec!["1","5","4","2","3"];
        assert_eq!(unique(input_list), expected_output);

        let input_arr = [1,4,2,3,5];
        let expected_ouput = vec![1,4,2,3,5];
        assert_eq!(unique(input_arr), expected_ouput);
    }
}
