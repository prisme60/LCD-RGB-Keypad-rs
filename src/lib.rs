pub mod glyphes;

#[cfg(test)]
mod tests {
    use crate::glyphes;
    #[test]
    fn it_works() {
        let mut v = vec![];
        assert_eq!(glyphes::convert_msg("Coucou é", &mut v), "Coucou \x00");
        assert_eq!(
            glyphes::convert_msg("Coucou é super ô", &mut v),
            "Coucou \x00 super \x01"
        );
        //\e[LG0 04 08 0e 11 1f 10 0f 00;\e[LG1 04 0a 0e 11 11 11 0e 00;
        assert_eq!(
            glyphes::generate_glyphes_string(&v),
            "\x1b[LG004080e111f100f00;\x1b[LG1040a0e1111110e00;"
        );
        println!("content : {:?}", &v);
    }
}
