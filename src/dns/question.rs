pub fn get_query_question(url: &str) -> Vec<u8> {
    let mut query_out = Vec::new();

    for word in url.split(".") {
        let word_count = word.len();

        query_out.push(word_count as u8);
        query_out.extend_from_slice(word.as_bytes());
    }
    query_out.push(0);
    query_out
}

// pub fn query_bytes(url: &str) -> Vec<u8> {
//     let mut question_u8: Vec<u8>;
//
//     // url.as_bytes().iter().for_each(|x| {
//     //     question_u8.push(*x);
//     // });
//
//     question_u8 = url.as_bytes().to_vec();
//     question_u8.push(0);
//     question_u8
// }
