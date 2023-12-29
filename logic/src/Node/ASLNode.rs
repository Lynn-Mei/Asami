struct ASLNode{
	content: vec<NodeType>,
	color: String,
	size: u32,
	mark: char,	
}

impl ASLNode{
	fn to_sxml() -> String {
		let mut node = String::from("<asl ");
		
		// to implement
		
		node.push_str(">");
		node.push_str(text.to_sxml());
		node.push_str("</asl>");
	}
	
	fn push_inner(NodeType node){
		content.push(node);
	}
	
}