#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    use crate::core::destination::Destination;
    use crate::core::ext::PathBufExt;
    use crate::core::system::home_dir;

    #[test]
    fn destination() {
        let home = home_dir().to_string();
        let in_exp = &[
            ("".to_string(), format!("{home}/Subpath/template.png")),
            (".".to_string(), "./template.png".to_string()),
            ("./".to_string(), "./template.png".to_string()),
            ("./name".to_string(), "./name.png".to_string()),
            ("./name.png".to_string(), "./name.png".to_string()),
            ("./stuff".to_string(), "./stuff/template.png".to_string()),
            ("./stuff/".to_string(), "./stuff/template.png".to_string()),
            ("./stuff/name".to_string(), "./stuff/name.png".to_string()),
            ("./stuff/name.png".to_string(), "./stuff/name.png".to_string()),
            ("~".to_string(), format!("{home}/template.png")),
            ("~/".to_string(), format!("{home}/template.png")),
            ("~/name".to_string(), format!("{home}/name.png")),
            ("~/name.png".to_string(), format!("{home}/name.png")),
            ("~/stuff".to_string(), format!("{home}/stuff/template.png")),
            ("~/stuff/".to_string(), format!("{home}/stuff/template.png")),
            ("~/stuff/name".to_string(), format!("{home}/stuff/name.png")),
            ("~/stuff/name.png".to_string(), format!("{home}/stuff/name.png")),
            (format!("{home}/stuff"), format!("{home}/stuff/template.png")),
            (format!("{home}/stuff/"), format!("{home}/stuff/template.png")),
            (format!("{home}/stuff/name"), format!("{home}/stuff/name.png")),
            (format!("{home}/stuff/name.png"), format!("{home}/stuff/name.png")),
            ("name".to_string(), format!("{home}/Subpath/name.png")),
            ("name.png".to_string(), format!("{home}/Subpath/name.png")),
            ("stuff/name".to_string(), format!("{home}/Subpath/stuff/name.png")),
            ("stuff/name.png".to_string(), format!("{home}/Subpath/stuff/name.png")),
            (".png".to_string(), format!("{home}/Subpath/.png")),
        ];
        let expected = in_exp.iter().map(|(_,it)| it.clone()).collect::<Vec<String>>();
        let output = in_exp.iter().map(|(input, expected)| {
            let output = input.clone()
                .dst_with_parent("~/Subpath/")
                .join("template.png")
                .to_string();
            let equals = output == *expected;
            let expected = if equals { String::new() } else { expected.clone() };
            println!("{input} -> {output} ? {equals} {expected}");
            output
        }).collect::<Vec<String>>();
        assert_equal(output, expected);
    }
}


























