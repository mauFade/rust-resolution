use move_zeros::move_zeros;

// pub mod one_edit_away;
// pub mod palindrome_number;
// pub mod retroactive_string;
// pub mod string_compression;
// pub mod unique_characters;
// pub mod enum_func;
// pub mod tuple;
// pub mod payment_type;
// pub mod struct_func;
// pub mod reverse_string;
// pub mod is_anagram;
// pub mod is_palindrome;
// pub mod rotate_arr;
// pub mod impl_keyword;
// pub mod kggest_num;
// pub mod max_profit;
pub mod move_zeros;

fn main() {
    // unique_characters::unique_characters("cateto");
    // retroactive_string::retroactive_string("tab", "mauricio");
    // one_edit_away::one_edit_away("mau", "mar");
    // string_compression::string_compression("Maaaaaaaauuuuu");
    // palindrome_number::palindrome_number(321);
    // tuple::tuple();
    // enum_func::enum_fn();
    // payment_type::payment_type(payment_type::Payment::CreditCard(true, 32.0));
    // struct_func::struct_func();
    // reverse_string::reverse_string("paralelepipedo".to_string())

    // let a = is_anagram::is_anagram("bad credit", "debit card");
    // let a = is_palindrome::is_palindrome("arara");
    // println!("{}", a)

    // rotate_arr::rotate_arr([1, 2, 3, 4, 5, 6, 7], 2);

    // impl_keyword::user_data("Mau".to_string(), "teste@test.com".to_string(), true);
    // kggest_num::kggest_num(vec![1, 7, 3, 9, 5, 10, 4], 3);
    // max_profit::max_profit([7, 1, 5, 3, 6, 4].to_vec());
    move_zeros(
        [
            15, 37, 1, 2, 1, 2, 9, 81, 103, 0, 67, 81, 0, 86, 278, 0, 276, 2874, 8894, 0,
        ]
        .to_vec(),
    );
}
