#![crate_name = "foo"]

// @has foo/struct.Foo.html
// @has - '//h2[@id="fields"]' 'Tuple Fields'
// @has - '//h3[@class="sidebar-title"]/a[@href="#fields"]' 'Tuple Fields'
// @has - '//*[@id="structfield.0"]' '0: u32'
// @has - '//*[@id="main"]/div[@class="docblock"]' 'hello'
// @!has - '//*[@id="structfield.1"]'
// @has - '//*[@id="structfield.2"]' '2: char'
// @has - '//*[@id="structfield.3"]' '3: i8'
// @has - '//*[@id="main"]/div[@class="docblock"]' 'not hello'
pub struct Foo(
    /// hello
    pub u32,
    char,
    pub char,
    /// not hello
    pub i8,
);

// @has foo/enum.Bar.html
// @has - '//pre[@class="rust enum"]' 'BarVariant(String),'
// @matches - '//*[@id="variant.BarVariant.fields"]/h4' '^Tuple Fields$'
// @has - '//*[@id="variant.BarVariant.field.0"]' '0: String'
// @has - '//*[@id="variant.BarVariant.fields"]//*[@class="docblock"]' 'Hello docs'
// @matches - '//*[@id="variant.FooVariant.fields"]/h4' '^Fields$'
pub enum Bar {
    BarVariant(
        /// Hello docs
        String
    ),
    FooVariant {
       /// hello
       x: u32,
    },
}
