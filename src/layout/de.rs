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
        let mut i = IterAtom::new(s, "gr_text")?;
        let value = i.s("value")?;
        let mut layer = footprint::Layer::default();
        let mut tstamp = None;
        let mut at = footprint::At::default();
        let mut effects = footprint::Effects::default();
        for x in i.iter {
            let elem = from_sexp(x)?;
            match elem {
                GrElement::At(x) => at = x,
                GrElement::Layer(x) => layer = x,
                GrElement::TStamp(x) => tstamp = Some(x),
                GrElement::Effects(x) => effects = x,
                _ => (), // TODO
            }
        }
        Ok(GrText {
            value: value,
            at: at,
            layer: layer,
            effects: effects,
            tstamp: tstamp,
        })
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
        let i = IterAtom::new(s, "gr_line")?;
        let mut start = footprint::Xy::new_empty(footprint::XyType::Start);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut angle = 0.0_f64;
        let mut layer = footprint::Layer::default();
        let mut width = 0.0_f64;
        let mut tstamp = None;
        for x in i.iter {
            let elem = from_sexp(x)?;
            match elem {
                GrElement::Start(x) => start = x,
                GrElement::End(x) => end = x,
                GrElement::Angle(x) => angle = x,
                GrElement::Layer(x) => layer = x,
                GrElement::TStamp(x) => tstamp = Some(x),
                GrElement::Width(x) => width = x,
                _ => (), // TODO
            }
        }
        Ok(GrLine {
            start: start,
            end: end,
            angle: angle,
            layer: layer,
            width: width,
            tstamp: tstamp,
        })
    }
}

impl FromSexp for GrArc {
    fn from_sexp(s: &Sexp) -> SResult<GrArc> {
        let i = IterAtom::new(s, "gr_arc")?;
        let mut start = footprint::Xy::new_empty(footprint::XyType::Start);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut angle = 0.0_f64;
        let mut layer = footprint::Layer::default();
        let mut width = 0.0_f64;
        let mut tstamp = None;
        for x in i.iter {
            let elem = from_sexp(x)?;
            match elem {
                GrElement::Start(x) => start = x,
                GrElement::End(x) => end = x,
                GrElement::Angle(x) => angle = x,
                GrElement::Layer(x) => layer = x,
                GrElement::TStamp(x) => tstamp = Some(x),
                GrElement::Width(x) => width = x,
                _ => (), // TODO
            }
        }
        Ok(GrArc {
            start: start,
            end: end,
            angle: angle,
            layer: layer,
            width: width,
            tstamp: tstamp,
        })
    }
}

impl FromSexp for GrCircle {
    fn from_sexp(s: &Sexp) -> SResult<GrCircle> {
        let i = IterAtom::new(s, "gr_circle")?;
        let mut center = footprint::Xy::new_empty(footprint::XyType::Center);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut layer = footprint::Layer::default();
        let mut width = 0.0_f64;
        let mut tstamp = None;
        for x in i.iter {
            let elem = from_sexp(x)?;
            match elem {
                GrElement::Center(x) => center = x,
                GrElement::End(x) => end = x,
                GrElement::Layer(x) => layer = x,
                GrElement::TStamp(x) => tstamp = Some(x),
                GrElement::Width(x) => width = x,
                _ => (), // TODO
            }
        }
        Ok(GrCircle {
            center: center,
            end: end,
            layer: layer,
            width: width,
            tstamp: tstamp,
        })
    }
}


impl FromSexp for Dimension {
    fn from_sexp(s: &Sexp) -> SResult<Dimension> {
        let mut i = IterAtom::new(s, "dimension")?;
        let name = i.s("name")?;
        let width = i.f_in_list("width")?;
        let layer = i.t("layer")?;
        let tstamp = i.maybe_s_in_list("tstamp");
        let text = i.t("text")?;
        let feature1 = i.t_in_list("feature1")?;
        let feature2 = i.t_in_list("feature2")?;
        let crossbar = i.t_in_list("crossbar")?;
        let arrow1a = i.t_in_list("arrow1a")?;
        let arrow1b = i.t_in_list("arrow1b")?;
        let arrow2a = i.t_in_list("arrow2a")?;
        let arrow2b = i.t_in_list("arrow2b")?;
        i.close(Dimension {
            name: name,
            width: width,
            layer: layer,
            tstamp: tstamp,
            text: text,
            feature1: feature1,
            feature2: feature2,
            crossbar: crossbar,
            arrow1a: arrow1a,
            arrow1b: arrow1b,
            arrow2a: arrow2a,
            arrow2b: arrow2b,
        })
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
        let priority = match i.maybe_i_in_list("priority") {
            Some(p) => p as u64,
            None => 0_u64,
        };
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
        let h = Hatch {
            style: i.s("style")?,
            pitch: i.f("pitch")?,
        };
        i.close(h)
    }
}

impl FromSexp for ConnectPads {
    fn from_sexp(s: &Sexp) -> SResult<ConnectPads> {
        let mut i = IterAtom::new(s, "connect_pads")?;
        let connection = i.maybe_s();
        let clearance = i.f_in_list("clearance")?;
        i.close(ConnectPads {
            connection: connection,
            clearance: clearance,
        })
    }
}
impl FromSexp for Keepout {
    fn from_sexp(s: &Sexp) -> SResult<Keepout> {
        let mut i = IterAtom::new(s, "keepout")?;
        let tracks = !i.s_in_list("tracks")?.starts_with("not");
        let vias = !i.s_in_list("vias")?.starts_with("not");
        let copperpour = !i.s_in_list("copperpour")?.starts_with("not");
        i.close(Keepout {
            tracks: tracks,
            vias: vias,
            copperpour: copperpour,
        })
    }
}

//  (fill yes (arc_segments 16) (thermal_gap 0.508) (thermal_bridge_width 0.508))
impl FromSexp for Fill {
    fn from_sexp(s: &Sexp) -> SResult<Fill> {
        let mut i = IterAtom::new(s, "fill")?;
        let filled = i.maybe_s().is_some();
        let mode = i.maybe_s_in_list("mode").is_some();
        let arc_segments = i.i_in_list("arc_segments")?;
        let thermal_gap = i.f_in_list("thermal_gap")?;
        let thermal_bridge_width = i.f_in_list("thermal_bridge_width")?;
        let smoothing = i.maybe_s_in_list("smoothing");
        let radius = i.maybe_f_in_list("radius").unwrap_or(0.0);
        Ok(Fill {
            filled: filled,
            segment: mode,
            arc_segments: arc_segments,
            thermal_gap: thermal_gap,
            thermal_bridge_width: thermal_bridge_width,
            smoothing: smoothing,
            corner_radius: radius,
        })
    }
}

impl FromSexp for Segment {
    fn from_sexp(s: &Sexp) -> SResult<Segment> {
        let i = IterAtom::new(s, "segment")?;
        let mut start = footprint::Xy::new_empty(footprint::XyType::Start);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut layer = footprint::Layer::default();
        let mut width = 0.0_f64;
        let mut tstamp = None;
        let mut net = 0;
        let mut status = None;
        // TODO: perhaps we can get rid of GrElement now we have IterAtom...
        for x in i.iter {
            let elem = from_sexp(x)?;
            match elem {
                GrElement::Start(x) => start = x,
                GrElement::End(x) => end = x,
                GrElement::Layer(x) => layer = x,
                GrElement::TStamp(x) => tstamp = Some(x),
                GrElement::Width(x) => width = x,
                GrElement::Net(x) => net = x,
                GrElement::Status(x) => status = Some(x),
                _ => (), // TODO
            }
        }
        Ok(Segment {
            start: start,
            end: end,
            width: width,
            layer: layer,
            net: net,
            tstamp: tstamp,
            status: status,
        })
    }
}
impl FromSexp for Via {
    fn from_sexp(s: &Sexp) -> SResult<Via> {
        let i = IterAtom::new(s, "via")?;
        let mut at = footprint::At::default();
        let mut size = 0.0_f64;
        let mut drill = 0.0_f64;
        let mut layers = footprint::Layers::default();
        let mut net = 0;
        for x in i.iter {
            let elem = from_sexp(x)?;
            match elem {
                GrElement::At(x) => at = x,
                GrElement::Size(x) => size = x,
                GrElement::Net(x) => net = x,
                GrElement::Drill(x) => drill = x,
                GrElement::Layers(x) => layers = x,
                _ => (), // TODO
            }
        }
        Ok(Via {
            at: at,
            size: size,
            drill: drill,
            layers: layers,
            net: net,
        })
    }
}

impl FromSexp for Layout {
    fn from_sexp(s: &Sexp) -> SResult<Layout> {
        let i = IterAtom::new(s, "kicad_pcb")?;
        let mut layout = Layout::default();
        // TODO: read in order instead of matching on iter
        for e in i.iter {
            match &(e.list_name()?)[..] {
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
