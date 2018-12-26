pub mod glyphes;

#[cfg(test)]
mod tests {
    use crate::glyphes;
    #[test]
    fn it_works() {
        let mut v = vec!();
        assert_eq!(glyphes::convert_msg("Coucou é", &mut v), "Coucou \x00");
        assert_eq!(glyphes::convert_msg("Coucou é super ô", &mut v), "Coucou \x00 super \x01");
        assert_eq!(glyphes::convert_msg("Coucou é super ô", &mut v), "Coucou \x00 super \x01");
        println!("content : {:?}", &v);
    }
}
