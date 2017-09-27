// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

// from parent
use footprint;
use wrap;
use Sexp;
use symbolic_expressions::iteratom::*;

use layout::data::*;

// TODO: switch more to IterAtom like in footprint/de.rs

// TODO: get rid of GrElement, using IterAtom allows dealing
// with the elements cleanly
enum GrElement {
    Start(footprint::Xy),
    End(footprint::Xy),
    Center(footprint::Xy),
    Angle(f64),
    Layer(footprint::Layer),
    Width(f64),
    Size(f64),
    Drill(f64),
    TStamp(String),
    At(footprint::At),
    Effects(footprint::Effects),
    Net(i64),
    Status(String),
    Layers(footprint::Layers),
}

struct Version(i64);

impl FromSexp for Version {
    fn from_sexp(s: &Sexp) -> SResult<Version> {
        let mut i = IterAtom::new(s, "version")?;
        let v = Version(i.i("value")?);
        i.close(v)
    }
}

struct Page(String);

impl FromSexp for Page {
    fn from_sexp(s: &Sexp) -> SResult<Page> {
        let mut i = IterAtom::new(s, "page")?;
        let p = Page(i.s("value")?);
        i.close(p)
    }
}

struct Polygon(footprint::Pts);

impl FromSexp for Polygon {
    fn from_sexp(s: &Sexp) -> SResult<Polygon> {
        let mut i = IterAtom::new(s, "polygon")?;
        let p = Polygon(i.t("pts")?);
        i.close(p)
    }
}

struct FilledPolygon(footprint::Pts);

impl FromSexp for FilledPolygon {
    fn from_sexp(s: &Sexp) -> SResult<FilledPolygon> {
        let mut i = IterAtom::new(s, "filled_polygon")?;
        let f = FilledPolygon(i.t("pts")?);
        i.close(f)
    }
}

struct FillSegments(footprint::Pts);

impl FromSexp for FillSegments {
    fn from_sexp(s: &Sexp) -> SResult<FillSegments> {
        let mut i = IterAtom::new(s, "fill_segments")?;
        Ok(FillSegments(i.t("pts")?))
    }
}

impl FromSexp for Net {
    fn from_sexp(s: &Sexp) -> SResult<Net> {
        let mut i = IterAtom::new(s, "net")?;
        let num = i.i("num")?;
        let name = i.s("name")?;
        i.close(Net {
            name: name.into(),
            num: num,
        })
    }
}

impl FromSexp for Host {
    fn from_sexp(s: &Sexp) -> SResult<Host> {
        let mut i = IterAtom::new(s, "host")?;
        let tool = i.s("tool")?;
        let build = i.s("build")?;
        i.close(Host {
            tool: tool,
            build: build,
        })
    }
}

impl FromSexp for General {
    fn from_sexp(s: &Sexp) -> SResult<General> {
        let mut i = IterAtom::new(s, "general")?;
        let links = i.i_in_list("links")?;
        let no_connects = i.i_in_list("no_connects")?;
        let area: Area = i.t("area")?;
        let thickness = i.f_in_list("thickness")?;
        let drawings = i.i_in_list("drawings")?;
        let tracks = i.i_in_list("tracks")?;
        let zones = i.i_in_list("zones")?;
        let modules = i.i_in_list("modules")?;
        let nets = i.i_in_list("nets")?;
        i.close(General {
            links: links,
            no_connects: no_connects,
            area: area,
            thickness: thickness,
            drawings: drawings,
            tracks: tracks,
            zones: zones,
            modules: modules,
            nets: nets,
        })
    }
}

impl FromSexp for Area {
    fn from_sexp(s: &Sexp) -> SResult<Area> {
        let mut i = IterAtom::new(s, "area")?;
        let a = Area {
            x1: i.f("x1")?,
            y1: i.f("y1")?,
            x2: i.f("x2")?,
            y2: i.f("y2")?,
        };
        i.close(a)
    }
}

struct LayerVec(Vec<Layer>);

impl FromSexp for LayerVec {
    fn from_sexp(s: &Sexp) -> SResult<LayerVec> {
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
    fn from_sexp(s: &Sexp) -> SResult<Layer> {
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
    fn from_sexp(s: &Sexp) -> SResult<LayerType> {
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
    fn from_sexp(s: &Sexp) -> SResult<SetupElement> {
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
    fn from_sexp(s: &Sexp) -> SResult<NetClass> {
        fn parse(e: &Sexp, name: &str) -> SResult<f64> {
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
    fn from_sexp(s: &Sexp) -> SResult<Setup> {
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
    fn from_sexp(s: &Sexp) -> SResult<GrText> {
        let mut t = GrText::default();
        let mut i = IterAtom::new(s, "gr_text")?;
        t.value = i.s("value")?;
        t.at = i.t("at")?;
        t.layer = i.t("layer")?;
        t.effects = i.t("effects")?;
        t.tstamp = i.maybe_s_in_list("tstamp");
        i.close(t)
    }
}

impl FromSexp for GrElement {
    fn from_sexp(s: &Sexp) -> SResult<GrElement> {
        match &(s.list_name()?)[..] {
            "start" => wrap(s, from_sexp, GrElement::Start),
            "end" => wrap(s, from_sexp, GrElement::End),
            "center" => wrap(s, from_sexp, GrElement::Center),
            "angle" => {
                let l2 = s.named_value_f("angle")?;
                Ok(GrElement::Angle(l2))
            }
            "layer" => wrap(s, from_sexp, GrElement::Layer),
            "width" => {
                let l2 = s.named_value_f("width")?;
                Ok(GrElement::Width(l2))
            }
            "size" => {
                let l2 = s.named_value_f("size")?;
                Ok(GrElement::Size(l2))
            }
            "drill" => {
                let l2 = s.named_value_f("drill")?;
                Ok(GrElement::Drill(l2))
            }
            "tstamp" => {
                let l2 = s.named_value_s("tstamp")?;
                Ok(GrElement::TStamp(l2))
            }
            "status" => {
                let l2 = s.named_value_s("status")?;
                Ok(GrElement::Status(l2))
            }
            "net" => {
                let l2 = s.named_value_i("net")?;
                Ok(GrElement::Net(l2))
            }
            "at" => wrap(s, from_sexp, GrElement::At),
            "layers" => wrap(s, from_sexp, GrElement::Layers),
            "effects" => wrap(s, from_sexp, GrElement::Effects),
            x => Err(format!("unknown element {} in {}", x, s).into()),
        }
    }
}


impl FromSexp for GrLine {
    fn from_sexp(s: &Sexp) -> SResult<GrLine> {
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
    fn from_sexp(s: &Sexp) -> SResult<GrArc> {
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
    fn from_sexp(s: &Sexp) -> SResult<GrCircle> {
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
    fn from_sexp(s: &Sexp) -> SResult<Dimension> {
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
    fn from_sexp(s: &Sexp) -> SResult<Zone> {
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
    fn from_sexp(s: &Sexp) -> SResult<Hatch> {
        let mut i = IterAtom::new(s, "hatch")?;
        let mut h = Hatch::default();
        h.style = i.s("style")?;
        h.pitch = i.f("pitch")?;
        i.close(h)
    }
}

impl FromSexp for ConnectPads {
    fn from_sexp(s: &Sexp) -> SResult<ConnectPads> {
        let mut connect_pads = ConnectPads::default();
        let mut i = IterAtom::new(s, "connect_pads")?;
        connect_pads.connection = i.maybe_s();
        connect_pads.clearance = i.f_in_list("clearance")?;
        i.close(connect_pads)
    }
}
impl FromSexp for Keepout {
    fn from_sexp(s: &Sexp) -> SResult<Keepout> {
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
    fn from_sexp(s: &Sexp) -> SResult<Fill> {
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
    fn from_sexp(s: &Sexp) -> SResult<Segment> {
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
    fn from_sexp(s: &Sexp) -> SResult<Via> {
        let mut i = IterAtom::new(s, "via")?;
        let mut via = Via::default();
        via.blind = i.maybe_literal_s("blind").is_some();
        via.micro = i.maybe_literal_s("micro").is_some();
        via.at = i.t("at")?;
        via.size = i.f_in_list("size")?;
        via.drill = i.maybe_f_in_list("drill").unwrap_or(0.0);
        via.layers = i.t("layers")?;
        via.net = i.maybe_i_in_list("net").unwrap_or(0);
        i.close(via)
    }
}

impl FromSexp for Layout {
    fn from_sexp(s: &Sexp) -> SResult<Layout> {
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
