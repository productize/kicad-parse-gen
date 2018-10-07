// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

// from parent
use footprint;
use wrap;
use Sexp;
use symbolic_expressions::iteratom::*;
use symbolic_expressions::SexpError;

use layout::data::*;

// TODO: switch more to IterAtom like in footprint/de.rs

struct Version(i64);

impl FromSexp for Version {
    fn from_sexp(s: &Sexp) -> Result<Version, SexpError> {
        let mut i = IterAtom::new(s, "version")?;
        let v = Version(i.i("value")?);
        i.close(v)
    }
}

struct Page(String);

impl FromSexp for Page {
    fn from_sexp(s: &Sexp) -> Result<Page, SexpError> {
        let mut i = IterAtom::new(s, "page")?;
        let p = Page(i.s("value")?);
        i.close(p)
    }
}

struct Polygon(footprint::Pts);

impl FromSexp for Polygon {
    fn from_sexp(s: &Sexp) -> Result<Polygon, SexpError> {
        let mut i = IterAtom::new(s, "polygon")?;
        let p = Polygon(i.t("pts")?);
        i.close(p)
    }
}

struct FilledPolygon(footprint::Pts);

impl FromSexp for FilledPolygon {
    fn from_sexp(s: &Sexp) -> Result<FilledPolygon, SexpError> {
        let mut i = IterAtom::new(s, "filled_polygon")?;
        let f = FilledPolygon(i.t("pts")?);
        i.close(f)
    }
}

struct FillSegments(footprint::Pts);

impl FromSexp for FillSegments {
    fn from_sexp(s: &Sexp) -> Result<FillSegments, SexpError> {
        let mut i = IterAtom::new(s, "fill_segments")?;
        let f = FillSegments(i.t("pts")?);
        i.close(f)
    }
}

impl FromSexp for Net {
    fn from_sexp(s: &Sexp) -> Result<Net, SexpError> {
        let mut net = Net::default();
        let mut i = IterAtom::new(s, "net")?;
        net.num = i.i("num")?;
        net.name = i.s("name")?.into();
        i.close(net)
    }
}

impl FromSexp for Host {
    fn from_sexp(s: &Sexp) -> Result<Host, SexpError> {
        let mut host = Host::default();
        let mut i = IterAtom::new(s, "host")?;
        host.tool = i.s("tool")?;
        host.build = i.s("build")?;
        i.close(host)
    }
}

impl FromSexp for General {
    fn from_sexp(s: &Sexp) -> Result<General, SexpError> {
        let mut g = General::default();
        let mut i = IterAtom::new(s, "general")?;
        g.thickness = i.f_in_list("thickness")?;
        g.drawings = i.i_in_list("drawings")?;
        g.tracks = i.i_in_list("tracks")?;
        g.zones = i.i_in_list("zones")?;
        g.modules = i.i_in_list("modules")?;
        g.nets = i.i_in_list("nets")?;
        i.close(g)
    }
}

struct LayerVec(Vec<Layer>);

impl FromSexp for LayerVec {
    fn from_sexp(s: &Sexp) -> Result<LayerVec, SexpError> {
        let i = IterAtom::new(s, "layers")?;
        let mut v = vec![];
        for x in i.iter {
            let layer = from_sexp(x)?;
            v.push(layer)
        }
        Ok(LayerVec(v))
    }
}

impl FromSexp for Layer {
    fn from_sexp(s: &Sexp) -> Result<Layer, SexpError> {
        let mut i = IterAtom::new_nameless(s, "layer")?;
        let num = i.i("num")?;
        let layer = footprint::Layer::from_string(&i.s("layer")?)?;
        let layer_type = i.t("layer_type")?;
        let hide = i.maybe_s().is_some();
        i.close(Layer {
            num: num,
            layer: layer,
            layer_type: layer_type,
            hide: hide,
        })
    }
}

impl FromSexp for LayerType {
    fn from_sexp(s: &Sexp) -> Result<LayerType, SexpError> {
        let x = s.string()?;
        match &x[..] {
            "signal" => Ok(LayerType::Signal),
            "power" => Ok(LayerType::Power),
            "mixed" => Ok(LayerType::Mixed),
            "jumper" => Ok(LayerType::Jumper),
            "user" => Ok(LayerType::User),
            _ => Err(format!("unknown layertype {} in {}", x, s).into()),
        }
    }
}

impl FromSexp for SetupElement {
    fn from_sexp(s: &Sexp) -> Result<SetupElement, SexpError> {
        let mut i = IterAtom::new_nameless(s, "setup_element")?;
        let m = SetupElement {
            name: i.s("name")?,
            value1: i.s("value1")?,
            value2: i.maybe_s(),
        };
        i.close(m)
    }
}

impl FromSexp for NetClass {
    fn from_sexp(s: &Sexp) -> Result<NetClass, SexpError> {
        fn parse(e: &Sexp, name: &str) -> Result<f64, SexpError> {
            let mut i = IterAtom::new(e, name)?;
            let r = i.f("element")?;
            i.close(r)
        }
        let mut i = IterAtom::new(s, "net_class")?;
        let name = i.s("name")?;
        let desc = i.s("desc")?;
        let mut clearance = 0.1524;
        let mut trace_width = 0.2032;
        let mut via_dia = 0.675;
        let mut via_drill = 0.25;
        let mut uvia_dia = 0.508;
        let mut uvia_drill = 0.127;
        let mut diff_pair_gap = None;
        let mut diff_pair_width = None;
        let mut nets = vec![];
        for x in i.iter {
            let list_name = x.list_name()?;
            let xn = &list_name[..];
            match xn {
                "add_net" => {
                    let l1 = x.named_value_s("add_net")?;
                    nets.push(l1)
                }
                "clearance" => clearance = parse(x, xn)?,
                "trace_width" => trace_width = parse(x, xn)?,
                "via_dia" => via_dia = parse(x, xn)?,
                "via_drill" => via_drill = parse(x, xn)?,
                "uvia_dia" => uvia_dia = parse(x, xn)?,
                "uvia_drill" => uvia_drill = parse(x, xn)?,
                "diff_pair_gap" => diff_pair_gap = Some(parse(x, xn)?),
                "diff_pair_width" => diff_pair_width = Some(parse(x, xn)?),
                _ => return Err(format!("unknown net_class field {}", list_name).into()),
            }
        }
        let net_class = NetClass {
            name: name,
            desc: desc,
            clearance: clearance,
            via_dia: via_dia,
            via_drill: via_drill,
            uvia_dia: uvia_dia,
            uvia_drill: uvia_drill,
            diff_pair_gap: diff_pair_gap,
            diff_pair_width: diff_pair_width,
            nets: nets.into_iter().map(|x| x.into()).collect(),
            trace_width: trace_width,
        };
        Ok(net_class)
    }
}

impl FromSexp for Setup {
    fn from_sexp(s: &Sexp) -> Result<Setup, SexpError> {
        let mut elements = vec![];
        let mut pcbplotparams = vec![];
        let i = IterAtom::new(s, "setup")?;
        for v in i.iter {
            let n = v.list_name()?;
            if n == "pcbplotparams" {
                let i2 = IterAtom::new(v, "pcbplotparams")?;
                for y in i2.iter {
                    let p_e = from_sexp(y)?;
                    pcbplotparams.push(p_e)
                }
            } else {
                let setup_element = from_sexp(v)?;
                elements.push(setup_element)
            }
        }
        Ok(Setup {
            elements: elements,
            pcbplotparams: pcbplotparams,
        })
    }
}

// for some reason this needs to be in a subfunction or it doesn't work
fn parse_other(e: &Sexp) -> Element {
    debug!("Element::Other: {}", e);
    Element::Other(e.clone())
}

impl FromSexp for GrText {
    fn from_sexp(s: &Sexp) -> Result<GrText, SexpError> {
        let mut t = GrText::default();
        let mut i = IterAtom::new(s, "gr_text")?;
        t.value = i.s("value")?;
        t.at = i.t("at")?;
        t.layer = i.t("layer")?;
        t.tstamp = i.maybe_s_in_list("tstamp");
        t.effects = i.t("effects")?;
        i.close(t)
    }
}

impl FromSexp for GrLine {
    fn from_sexp(s: &Sexp) -> Result<GrLine, SexpError> {
        let mut l = GrLine::default();
        let mut i = IterAtom::new(s, "gr_line")?;
        l.start = i.t("start")?;
        l.end = i.t("end")?;
        l.angle = i.maybe_f_in_list("angle").unwrap_or(0.0);
        l.layer = i.t("layer")?;
        l.width = i.maybe_f_in_list("width").unwrap_or(0.0);
        l.tstamp = i.maybe_s_in_list("tstamp");
        i.close(l)
    }
}

impl FromSexp for GrArc {
    fn from_sexp(s: &Sexp) -> Result<GrArc, SexpError> {
        let mut a = GrArc::default();
        let mut i = IterAtom::new(s, "gr_arc")?;
        a.start = i.t("start")?;
        a.end = i.t("end")?;
        a.angle = i.f_in_list("angle")?;
        a.layer = i.t("layer")?;
        a.width = i.maybe_f_in_list("width").unwrap_or(0.0);
        a.tstamp = i.maybe_s_in_list("tstamp");
        // TODO: maybe status field?
        i.close(a)
    }
}

impl FromSexp for GrCircle {
    fn from_sexp(s: &Sexp) -> Result<GrCircle, SexpError> {
        let mut c = GrCircle::default();
        let mut i = IterAtom::new(s, "gr_circle")?;
        c.center = i.t("center")?;
        c.end = i.t("end")?;
        c.layer = i.t("layer")?;
        c.width = i.maybe_f_in_list("width").unwrap_or(0.0);
        c.tstamp = i.maybe_s_in_list("tstamp");
        // TODO: status field?
        i.close(c)
    }
}


impl FromSexp for Dimension {
    fn from_sexp(s: &Sexp) -> Result<Dimension, SexpError> {
        let mut d = Dimension::default();
        let mut i = IterAtom::new(s, "dimension")?;
        d.name = i.s("name")?;
        d.width = i.f_in_list("width")?;
        d.layer = i.t("layer")?;
        d.tstamp = i.maybe_s_in_list("tstamp");
        d.text = i.t("text")?;
        d.feature1 = i.t_in_list("feature1")?;
        d.feature2 = i.t_in_list("feature2")?;
        d.crossbar = i.t_in_list("crossbar")?;
        d.arrow1a = i.t_in_list("arrow1a")?;
        d.arrow1b = i.t_in_list("arrow1b")?;
        d.arrow2a = i.t_in_list("arrow2a")?;
        d.arrow2b = i.t_in_list("arrow2b")?;
        i.close(d)
    }
}

impl FromSexp for Zone {
    fn from_sexp(s: &Sexp) -> Result<Zone, SexpError> {
        let mut i = IterAtom::new(s, "zone")?;
        let net = i.i_in_list("net")?;
        let net_name = i.s_in_list("net_name")?;
        let layer = i.t("layer")?;
        let tstamp = i.s_in_list("tstamp")?;
        let hatch = i.t("hatch")?;
        let priority = i.maybe_i_in_list("priority").unwrap_or(0) as u64;
        let connect_pads = i.t("connect_pads")?;
        let min_thickness = i.f_in_list("min_thickness")?;
        let keepout = i.maybe_t();
        let fill = i.t("fill")?;
        let mut polygons = vec![];
        let mut filled_polygons = vec![];
        let mut fill_segments = None;
        let mut others = vec![];
        for x in i.iter {
            if let Ok(p) = Polygon::from_sexp(x) {
                polygons.push(p.0)
            } else if let Ok(p) = FilledPolygon::from_sexp(x) {
                filled_polygons.push(p.0)
            } else if let Ok(p) = FillSegments::from_sexp(x) {
                fill_segments = Some(p.0);
            } else {
                others.push(x.clone());
                debug!("'zone': not parsing {}", x);
            }
        }
        Ok(Zone {
            net: net,
            net_name: net_name.into(),
            layer: layer,
            tstamp: tstamp,
            hatch: hatch,
            priority: priority,
            connect_pads: connect_pads,
            min_thickness: min_thickness,
            keepout: keepout,
            fill: fill,
            polygons: polygons,
            filled_polygons: filled_polygons,
            fill_segments: fill_segments,
            other: others,
        })
    }
}

impl FromSexp for Hatch {
    fn from_sexp(s: &Sexp) -> Result<Hatch, SexpError> {
        let mut i = IterAtom::new(s, "hatch")?;
        let mut h = Hatch::default();
        h.style = i.s("style")?;
        h.pitch = i.f("pitch")?;
        i.close(h)
    }
}

impl FromSexp for ConnectPads {
    fn from_sexp(s: &Sexp) -> Result<ConnectPads, SexpError> {
        let mut connect_pads = ConnectPads::default();
        let mut i = IterAtom::new(s, "connect_pads")?;
        connect_pads.connection = i.maybe_s();
        connect_pads.clearance = i.f_in_list("clearance")?;
        i.close(connect_pads)
    }
}
impl FromSexp for Keepout {
    fn from_sexp(s: &Sexp) -> Result<Keepout, SexpError> {
        let mut keepout = Keepout::default();
        let mut i = IterAtom::new(s, "keepout")?;
        keepout.tracks = !i.s_in_list("tracks")?.starts_with("not");
        keepout.vias = !i.s_in_list("vias")?.starts_with("not");
        keepout.copperpour = !i.s_in_list("copperpour")?.starts_with("not");
        i.close(keepout)
    }
}

//  (fill yes (arc_segments 16) (thermal_gap 0.508) (thermal_bridge_width 0.508))
impl FromSexp for Fill {
    fn from_sexp(s: &Sexp) -> Result<Fill, SexpError> {
        let mut fill = Fill::default();
        let mut i = IterAtom::new(s, "fill")?;
        fill.filled = i.maybe_literal_s("yes").is_some();
        fill.segment = i.maybe_s_in_list("mode").is_some();
        fill.arc_segments = i.i_in_list("arc_segments")?;
        fill.thermal_gap = i.f_in_list("thermal_gap")?;
        fill.thermal_bridge_width = i.f_in_list("thermal_bridge_width")?;
        fill.smoothing = i.maybe_s_in_list("smoothing");
        fill.corner_radius = i.maybe_f_in_list("radius").unwrap_or(0.0);
        i.close(fill)
    }
}

// (segment (start 211 61.1) (end 211.1 61) (width 0.2032) (layer F.Cu) (net 20) [(tstamp 55A0DB7E)] [(status foo)])
impl FromSexp for Segment {
    fn from_sexp(s: &Sexp) -> Result<Segment, SexpError> {
        let mut i = IterAtom::new(s, "segment")?;
        let mut segment = Segment::default();
        segment.start = i.t("start")?;
        segment.end = i.t("end")?;
        segment.width = i.f_in_list("width")?;
        segment.layer = i.t("layer")?;
        segment.net = i.i_in_list("net")?;
        segment.tstamp = i.maybe_s_in_list("tstamp");
        segment.status = i.maybe_s_in_list("status");
        i.close(segment)
    }
}

// (via [blind] [micro] (at 132.1948 121.2202) (size 0.675) (drill 0.25) (layers F.Cu B.Cu) (net 19))
impl FromSexp for Via {
    fn from_sexp(s: &Sexp) -> Result<Via, SexpError> {
        let mut i = IterAtom::new(s, "via")?;
        let mut via = Via::default();
        via.blind = i.maybe_literal_s("blind").is_some();
        via.micro = i.maybe_literal_s("micro").is_some();
        via.at = i.t("at")?;
        via.size = i.f_in_list("size")?;
        via.drill = i.maybe_f_in_list("drill").unwrap_or(0.0);
        via.layers = i.t("layers")?;
        via.net = i.maybe_i_in_list("net").unwrap_or(0);
        via.tstamp = i.maybe_s_in_list("tstamp");
        i.close(via)
    }
}

impl FromSexp for Layout {
    fn from_sexp(s: &Sexp) -> Result<Layout, SexpError> {
        let i = IterAtom::new(s, "kicad_pcb")?;
        let mut layout = Layout::default();
        // TODO: read in order instead of matching on iter
        for e in i.iter {
            match (e.list_name()?).as_str() {
                "version" => layout.version = Version::from_sexp(e)?.0,
                "host" => layout.host = from_sexp(e)?,
                "general" => layout.general = from_sexp(e)?,
                "page" => layout.page = Page::from_sexp(e)?.0,
                "layers" => layout.layers = LayerVec::from_sexp(e)?.0,
                "module" => {
                    let module = wrap(e, from_sexp, Element::Module)?;
                    layout.elements.push(module)
                }
                "net" => {
                    let net = wrap(e, from_sexp, Element::Net)?;
                    layout.elements.push(net)
                }
                "net_class" => {
                    let nc = wrap(e, from_sexp, Element::NetClass)?;
                    layout.elements.push(nc)
                }
                "gr_text" => {
                    let g = wrap(e, from_sexp, Element::GrText)?;
                    layout.elements.push(g)
                }
                "gr_line" => {
                    let g = wrap(e, from_sexp, Element::GrLine)?;
                    layout.elements.push(g)
                }
                "gr_arc" => {
                    let g = wrap(e, from_sexp, Element::GrArc)?;
                    layout.elements.push(g)
                }
                "gr_circle" => {
                    let g = wrap(e, from_sexp, Element::GrCircle)?;
                    layout.elements.push(g)
                }
                "dimension" => {
                    let g = wrap(e, from_sexp, Element::Dimension)?;
                    layout.elements.push(g)
                }
                "zone" => {
                    let g = wrap(e, from_sexp, Element::Zone)?;
                    layout.elements.push(g)
                }
                "segment" => {
                    let g = wrap(e, from_sexp, Element::Segment)?;
                    layout.elements.push(g)
                }
                "via" => {
                    let g = wrap(e, from_sexp, Element::Via)?;
                    layout.elements.push(g)
                }
                "setup" => layout.setup = from_sexp(e)?,
                _ => {
                    // println!("unimplemented: {}", e);
                    layout.elements.push(parse_other(e))
                }
            }
        }
        Ok(layout)
    }
}
