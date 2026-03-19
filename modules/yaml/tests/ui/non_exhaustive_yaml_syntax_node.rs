use oni_comb_yaml::{CollectionStyle, YamlSyntaxNode, YamlSyntaxScalar};

fn main() {
  let node = YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain(String::new()));
  match node {
    YamlSyntaxNode::Scalar(_) => {}
    YamlSyntaxNode::Sequence {
      style: CollectionStyle::Flow,
      items: _,
    } => {}
    YamlSyntaxNode::Mapping {
      style: CollectionStyle::Flow,
      entries: _,
    } => {}
  }
}
