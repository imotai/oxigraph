#![no_main]

use libfuzzer_sys::fuzz_target;
use oxigraph_fuzz::count_triple_blank_nodes;
use oxrdf::graph::CanonicalizationAlgorithm;
use oxrdf::{Graph, Triple};
use oxrdfxml::{RdfXmlParser, RdfXmlSerializer};
use std::io::ErrorKind;

fn parse(
    data: &[u8],
    unchecked: bool,
) -> (
    Vec<Triple>,
    Vec<String>,
    Vec<(String, String)>,
    Option<String>,
) {
    let mut triples = Vec::new();
    let mut errors = Vec::new();
    let mut parser = RdfXmlParser::new();
    if unchecked {
        parser = parser.lenient();
    }
    let mut parser = parser.for_slice(data);
    for result in &mut parser {
        match result {
            Ok(triple) => triples.push(triple),
            Err(error) => errors.push(error.to_string()),
        }
    }
    (
        triples,
        errors,
        parser
            .prefixes()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        parser.base_iri().map(ToString::to_string),
    )
}

fuzz_target!(|data: &[u8]| {
    // We parse
    let (triples, errors, prefixes, base_iri) = parse(data, false);

    // We test also unchecked if valid
    let (triples_unchecked, errors_unchecked, _, _) = parse(data, true);
    if errors.is_empty() {
        assert!(errors_unchecked.is_empty());

        let bnodes_count = triples
            .iter()
            .map(|t| count_triple_blank_nodes(t.as_ref()))
            .sum::<usize>();
        if bnodes_count == 0 {
            assert_eq!(triples, triples_unchecked);
        } else if bnodes_count <= 4 {
            let mut graph_with_split = triples.iter().collect::<Graph>();
            let mut graph_unchecked = triples_unchecked.iter().collect::<Graph>();
            graph_with_split.canonicalize(CanonicalizationAlgorithm::Unstable);
            graph_unchecked.canonicalize(CanonicalizationAlgorithm::Unstable);
            assert_eq!(graph_with_split, graph_unchecked);
        }
    }

    // We serialize
    let mut serializer = RdfXmlSerializer::new();
    for (prefix_name, prefix_iri) in prefixes {
        serializer = serializer.with_prefix(prefix_name, prefix_iri).unwrap();
    }
    if let Some(base_iri) = base_iri {
        serializer = serializer.with_base_iri(base_iri).unwrap();
    }
    let mut serializer = serializer.for_writer(Vec::new());
    for triple in &triples {
        if let Err(e) = serializer.serialize_triple(triple) {
            if e.kind() == ErrorKind::InvalidInput {
                return; // Our serializer is strict and might reject some parsing outputs
            }
            panic!("Serialization failed with {e}")
        }
    }
    let new_serialization = serializer.finish().unwrap();

    // We parse the serialization
    let new_triples = RdfXmlParser::new()
        .for_slice(&new_serialization)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            format!(
                "Error on '{}' from {triples:?} based on '{}': {e}",
                String::from_utf8_lossy(&new_serialization),
                String::from_utf8_lossy(data)
            )
        })
        .unwrap();

    // We check the roundtrip has not changed anything
    assert_eq!(new_triples, triples);
});
