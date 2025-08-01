#![cfg(test)]

use anyhow::Result;
use oxigraph_testsuite::check_testsuite;

#[test]
fn rdf11_n_triples_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf11/rdf-n-triples/manifest.ttl",
        &[],
    )
}

#[test]
fn rdf12_n_triples_syntax_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf12/rdf-n-triples/syntax/manifest.ttl",
        &[],
    )
}
#[cfg(not(windows))] // Tests don't like git auto "\r\n" on Windows
#[test]
fn rdf12_n_triples_c14n_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf12/rdf-n-triples/c14n/manifest.ttl",
        &[],
    )
}

#[test]
fn rdf11_n_quads_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf11/rdf-n-quads/manifest.ttl",
        &[],
    )
}

#[test]
fn rdf12_n_quads_syntax_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf12/rdf-n-quads/syntax/manifest.ttl",
        &[],
    )
}

#[cfg(not(windows))] // Tests don't like git auto "\r\n" on Windows
#[test]
fn rdf12_n_quads_c14n_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf12/rdf-n-quads/c14n/manifest.ttl",
        &[],
    )
}

#[cfg(not(windows))] // Tests don't like git auto "\r\n" on Windows
#[test]
fn rdf11_turtle_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf11/rdf-turtle/manifest.ttl",
        &[],
    )
}

#[test]
fn rdf12_turtle_syntax_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf12/rdf-turtle/syntax/manifest.ttl",
        &[],
    )
}

#[test]
fn rdf12_turtle_eval_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf12/rdf-turtle/eval/manifest.ttl",
        &[],
    )
}

#[cfg(not(windows))] // Tests don't like git auto "\r\n" on Windows
#[test]
fn rdf11_trig_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf11/rdf-trig/manifest.ttl",
        &[],
    )
}

#[test]
fn rdf12_trig_syntax_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf12/rdf-trig/syntax/manifest.ttl",
        &[],
    )
}

#[test]
fn rdf12_trig_eval_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf12/rdf-trig/eval/manifest.ttl",
        &[],
    )
}

#[test]
fn rdf11_xml_w3c_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/rdf-tests/rdf/rdf11/rdf-xml/manifest.ttl",
        &[],
    )
}

#[test]
fn n3_parser_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/N3/tests/N3Tests/manifest-parser.ttl",
        &[],
    )
}

#[test]
fn n3_extended_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/N3/tests/N3Tests/manifest-extended.ttl",
        &[],
    )
}

#[cfg(not(windows))] // Tests don't like git auto "\r\n" on Windows
#[test]
fn n3_turtle_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/N3/tests/TurtleTests/manifest.ttl",
        &[],
    )
}

#[test]
fn jsonld_to_rdf_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/json-ld-api/tests/toRdf-manifest.jsonld",
        &[
            // relative IRI resolution discrepancies
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#t0122",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#t0123",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te062",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te091",
            // Weird @base IRI support
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tli12",
            // expandContext
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te077",
            // produceGeneralizedRdf
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#t0118",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te075",
            // non-normative - rdfDirection
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tdi09",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tdi10",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tdi11",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tdi12",
            // scoped context
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc001",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc002",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc003",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc004",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc005",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc006",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc007",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc008",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc009",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc010",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc011",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc012",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc013",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc014",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc015",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc016",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc017",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc018",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc019",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc020",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc021",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc022",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc023",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc024",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc025",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc026",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc027",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc028",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc030",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc031",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc032",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc033",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc034",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc035",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc036",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc037",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tc038",
            // @graph @container
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te079",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te080",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te081",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te082",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te083",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te085",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te086",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te087",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te093",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te094",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te095",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te096",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te097",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te098",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te099",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te100",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te101",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te102",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te103",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te104",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te105",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te106",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te107",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te108",
            // @nest
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ten01",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ten02",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ten03",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ten04",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ten05",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ten06",
            // @included
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tin01",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tin02",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tin03",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tin04",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tin05",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tin06",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tin07",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tin08",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tin09",
            // @container @id
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm001",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm002",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm003",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm005",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm011",
            // @container @type
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm004",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm006",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm007",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm008",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm012",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm017",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm018",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm019",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm020",
            // @container @ŋraph @index
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm013",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm014",
            // @container @graph @id
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm015",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tm016",
            // @nest
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tn001",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tn002",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tn003",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tn004",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tn005",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tn006",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tn007",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tn008",
            // @container @index
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpi05",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpi06",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpi07",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpi08",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpi09",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpi10",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpi11",
            // @protected
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr08",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr14",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr15",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr16",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr17",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr18",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr19",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr20",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr21",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr22",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr23",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr25",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr26",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr27",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr30",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr39",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr40",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tpr43",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#trt01",
            // @propagate
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tso05",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tso06",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ttn02",
            // useJCS
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs01",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs02",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs03",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs04",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs05",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs06",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs07",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs08",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs09",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs10",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs11",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs12",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs13",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs14",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs15",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs16",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs17",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs18",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs19",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs20",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs21",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs22",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#tjs23",
            // specVersion json-ld-1.0
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te026",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te071",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te115",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#te116",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ter02",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ter03",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ter24",
            "https://w3c.github.io/json-ld-api/tests/toRdf-manifest#ter32",
        ],
    )
}

#[test]
fn jsonld_to_rdf_streaming_testsuite() -> Result<()> {
    check_testsuite(
        "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest.jsonld",
        &[
            // We do not allow root @graph followed with other keys
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tv017",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tv019",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tv021",
            // @context in @context (JSON-LD 1.1)
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tv006",
            // relative IRI resolution discrepancies
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#t0122",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#t0123",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te062",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te091",
            // expandContext option
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te077",
            // normative option
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi09",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi10",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi11",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi12",
            // produceGeneralizedRdf option
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#t0118",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te068",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te075",
            // rdfDirection option
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi09",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi10",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi11",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi12",
            // specVersion option
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#t0118",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#t0124",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#t0125",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc001",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc002",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc003",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc004",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc005",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc006",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc007",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc008",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc009",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc010",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc011",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc012",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc013",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc014",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc015",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc016",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc017",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc018",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc019",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc020",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc021",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc022",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc023",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc024",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc025",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc026",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc027",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc028",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc030",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc031",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc032",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc033",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc034",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tc035",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi04",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi06",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi07",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi08",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi09",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi10",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi11",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tdi12",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te014",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te026",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te038",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te071",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te079",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te080",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te081",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te082",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te083",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te084",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te085",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te086",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te087",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te092",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te093",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te094",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te095",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te096",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te097",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te098",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te099",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te100",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te101",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te102",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te103",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te104",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te105",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te106",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te107",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te108",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te110",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te111",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te112",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te114",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te115",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te116",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te117",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te118",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te119",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te120",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te121",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te122",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te123",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te126",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te127",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te128",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te129",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#te130",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter53",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tec01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tec02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tem01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ten01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ten02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ten03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ten04",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ten05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ten06",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tep03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter24",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter32",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter43",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter44",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter48",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ter49",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tin01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tin02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tin03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tin04",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tin05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tin06",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tin07",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tin08",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tin09",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli04",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli06",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli07",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli08",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli09",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tli10",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm001",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm002",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm003",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm004",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm005",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm006",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm007",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm008",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm009",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm010",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm011",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm012",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm013",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm014",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm015",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm016",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm017",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm018",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm019",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tm020",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tn001",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tn002",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tn003",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tn004",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tn005",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tn006",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tn007",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tn008",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tp001",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tp002",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tp003",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tp004",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi04",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi06",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi07",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi08",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi09",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi10",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpi11",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr04",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr06",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr08",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr09",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr10",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr11",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr12",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr13",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr14",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr15",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr16",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr17",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr18",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr19",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr20",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr21",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr22",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr23",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr24",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr25",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr26",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr27",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr28",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr29",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr30",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr31",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr32",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr33",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr34",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr35",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr36",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr37",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr38",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr39",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tpr40",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#trt01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso06",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso07",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso08",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso09",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso10",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso11",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso12",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tso13",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#ttn02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#twf01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#twf02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#twf03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#twf04",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#twf05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#twf07",
            // useJCS option
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs01",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs02",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs03",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs04",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs05",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs06",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs07",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs08",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs09",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs10",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs11",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs12",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs13",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs14",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs15",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs16",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs17",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs18",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs19",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs20",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs21",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs22",
            "https://w3c.github.io/json-ld-streaming/tests/stream-toRdf-manifest#tjs23",
        ],
    )
}
