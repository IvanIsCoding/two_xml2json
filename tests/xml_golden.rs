use serde_json::{Value, json};
use two_xml2json::parse_xml;

macro_rules! test {
    ($name:ident, $xml:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let expected: Value = $expected;
            assert_eq!(parse_xml($xml).unwrap(), expected);
        }
    };
}

test!(
    xml_nested_elements,
    r#"<database>
  <host>localhost</host>
  <port>5432</port>
</database>"#,
    json!({"database": {"host": "localhost", "port": "5432"}})
);

test!(
    xml_array_of_elements,
    r#"<servers>
  <server>
    <ip>192.168.1.1</ip>
    <name>alpha</name>
  </server>
  <server>
    <ip>192.168.1.2</ip>
    <name>beta</name>
  </server>
</servers>"#,
    json!({
        "servers": {
            "server": [
                {"ip": "192.168.1.1", "name": "alpha"},
                {"ip": "192.168.1.2", "name": "beta"}
            ]
        }
    })
);

test!(
    xml_three_repeated_elements_append_to_array,
    r#"<servers>
  <server><ip>192.168.1.1</ip></server>
  <server><ip>192.168.1.2</ip></server>
  <server><ip>192.168.1.3</ip></server>
</servers>"#,
    json!({
        "servers": {
            "server": [
                {"ip": "192.168.1.1"},
                {"ip": "192.168.1.2"},
                {"ip": "192.168.1.3"}
            ]
        }
    })
);

test!(
    xml_with_attributes,
    r#"<user>
  <name>Alice</name>
  <email>alice@example.com</email>
</user>"#,
    json!({"user": {"name": "Alice", "email": "alice@example.com"}})
);

test!(
    xml_element_attributes,
    r#"<user id="42">
  <name>Alice</name>
</user>"#,
    json!({"user": {"$": {"id": "42"}, "name": "Alice"}})
);

test!(
    xml_simple_structure,
    r#"<point>
  <x>10</x>
  <y>20</y>
</point>"#,
    json!({"point": {"x": "10", "y": "20"}})
);

test!(
    xml_text_content,
    r#"<message>Hello, World!</message>"#,
    json!({"message": "Hello, World!"})
);

test!(
    xml_mixed_content,
    r#"<person>
  <name>Bob</name>
  <age>25</age>
</person>"#,
    json!({"person": {"name": "Bob", "age": "25"}})
);

test!(
    xml_to_pretty_sorted_json,
    r#"<root>
  <person>
    <name>Alice</name>
    <age>30</age>
  </person>
  <id>1</id>
</root>"#,
    json!({
        "root": {
            "person": {"name": "Alice", "age": "30"},
            "id": "1"
        }
    })
);

test!(xml_empty_root_element, r#"<empty/>"#, json!({"empty": ""}));

test!(
    xml_empty_child_element,
    r#"<config>
  <enabled/>
  <name>celq</name>
</config>"#,
    json!({"config": {"enabled": "", "name": "celq"}})
);

test!(
    xml_cdata_and_references,
    r#"<message><![CDATA[Use <tags>]]> &amp; &#65;</message>"#,
    json!({"message": "Use <tags> & A"})
);

test!(
    xml_mixed_text_and_child_preserves_text_key,
    r#"<article>Intro <title>XML</title> outro</article>"#,
    json!({"article": {"title": "XML", "_": "Intro  outro"}})
);

test!(
    xml_declaration_comments_and_processing_instructions,
    r#"<?xml version="1.0"?>
<!-- ignored -->
<root><?format compact?><value>ok</value></root>"#,
    json!({"root": {"value": "ok"}})
);
