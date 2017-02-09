// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

// from parent
use Result;
use footprint;
use wrap;
use Sexp;
use str_error;
use FromSexp;
use GrElement;
use from_sexp;
use IterAtom;

use layout::data::*;

// TODO: switch more to IterAtom like in footprint/de.rs

fn parse_version(e: &Sexp) -> Result<i64> {
    let l = e.slice_atom("version")?;
    l[0].i().map_err(From::from)
}

fn parse_page(e: &Sexp) -> Result<String> {
    let l = e.slice_atom("page")?;
    Ok(l[0].string()?.clone())
}

struct TStamp(String);

impl FromSexp for TStamp {
    fn from_sexp(s: &Sexp) -> Result<TStamp> {
        let mut i = IterAtom::new(s, "tstamp")?;
        Ok(TStamp(i.s("tstamp", "value")?))
    }
}

struct ZoneNetName(String);

impl FromSexp for ZoneNetName {
    fn from_sexp(s: &Sexp) -> Result<ZoneNetName> {
        let mut i = IterAtom::new(s, "net_name")?;
        Ok(ZoneNetName(i.s("net_name", "value")?))
    }
}

struct MinThickness(f64);

impl FromSexp for MinThickness {
    fn from_sexp(s: &Sexp) -> Result<MinThickness> {
        let mut i = IterAtom::new(s, "min_thickness")?;
        Ok(MinThickness(i.f("min_thickness", "value")?))
    }
}

struct Priority(i64);

impl FromSexp for Priority {
    fn from_sexp(s: &Sexp) -> Result<Priority> {
        let mut i = IterAtom::new(s, "priority")?;
        Ok(Priority(i.i("priority", "value")?))
    }
}

struct ZoneNet(i64);

impl FromSexp for ZoneNet {
    fn from_sexp(s: &Sexp) -> Result<ZoneNet> {
        let mut i = IterAtom::new(s, "net")?;
        Ok(ZoneNet(i.i("net", "value")?))
    }
}

struct FillMode(String);

impl FromSexp for FillMode {
    fn from_sexp(s: &Sexp) -> Result<FillMode> {
        let mut i = IterAtom::new(s, "mode")?;
        Ok(FillMode(i.s("mode", "value")?))
    }
}

struct FillArcSegments(i64);

impl FromSexp for FillArcSegments {
    fn from_sexp(s: &Sexp) -> Result<FillArcSegments> {
        let mut i = IterAtom::new(s, "arc_segments")?;
        Ok(FillArcSegments(i.i("arc_segments", "value")?))
    }
}

struct FillThermalGap(f64);

impl FromSexp for FillThermalGap {
    fn from_sexp(s: &Sexp) -> Result<FillThermalGap> {
        let mut i = IterAtom::new(s, "thermal_gap")?;
        Ok(FillThermalGap(i.f("thermal_gap", "value")?))
    }
}

struct FillThermalBridgeWidth(f64);

impl FromSexp for FillThermalBridgeWidth {
    fn from_sexp(s: &Sexp) -> Result<FillThermalBridgeWidth> {
        let mut i = IterAtom::new(s, "thermal_bridge_width")?;
        Ok(FillThermalBridgeWidth(i.f("thermal_bridge_width", "value")?))
    }
}

struct FillRadius(f64);

impl FromSexp for FillRadius {
    fn from_sexp(s: &Sexp) -> Result<FillRadius> {
        let mut i = IterAtom::new(s, "radius")?;
        Ok(FillRadius(i.f("radius", "value")?))
    }
}

struct FillSmoothing(String);

impl FromSexp for FillSmoothing {
    fn from_sexp(s: &Sexp) -> Result<FillSmoothing> {
        let mut i = IterAtom::new(s, "smoothing")?;
        Ok(FillSmoothing(i.s("smoothing", "value")?))
    }
}

impl FromSexp for Net {
    fn from_sexp(s: &Sexp) -> Result<Net> {
        let l = s.slice_atom_num("net", 2)?;
        let num = l[0].i()?;
        let name = l[1].string()?.clone();
        Ok(Net {
            name: name,
            num: num,
        })
    }
}

impl FromSexp for Host {
    fn from_sexp(s: &Sexp) -> Result<Host> {
        let l = s.slice_atom_num("host", 2)?;
        let tool = l[0].string()?.clone();
        let build = l[1].string()?.clone();
        Ok(Host {
            tool: tool,
            build: build,
        })
    }
}

impl FromSexp for General {
    fn from_sexp(s: &Sexp) -> Result<General> {
        let l = s.slice_atom_num("general", 9)?;
        let links = l[0].named_value_i("links")?;
        let no_connects = l[1].named_value_i("no_connects")?;
        let area = from_sexp(&l[2])?;
        let thickness = l[3].named_value_f("thickness")?;
        let drawings = l[4].named_value_i("drawings")?;
        let tracks = l[5].named_value_i("tracks")?;
        let zones = l[6].named_value_i("zones")?;
        let modules = l[7].named_value_i("modules")?;
        let nets = l[8].named_value_i("nets")?;
        Ok(General {
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
    fn from_sexp(s: &Sexp) -> Result<Area> {
        let l = s.slice_atom_num("area", 4)?;
        let x1 = l[0].f()?;
        let y1 = l[1].f()?;
        let x2 = l[2].f()?;
        let y2 = l[3].f()?;
        Ok(Area {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
        })
    }
}

impl FromSexp for Vec<Layer> {
    fn from_sexp(s: &Sexp) -> Result<Vec<Layer>> {
        let mut v = vec![];
        let l = s.slice_atom("layers")?;
        for x in l {
            let layer = from_sexp(&x)?;
            v.push(layer)
        }
        Ok(v)
    }
}

impl FromSexp for Layer {
    fn from_sexp(s: &Sexp) -> Result<Layer> {
        let l = s.list()?;
        // println!("making layer from {}", s);
        if l.len() != 3 && l.len() != 4 {
            return str_error(format!("expecting 3 or 4 elements in layer: {}", s));
        }
        let num = l[0].i()?;
        let layer = footprint::Layer::from_string(l[1].string()?)?;
        let layer_type = from_sexp(&l[2])?;
        let hide = if l.len() == 3 {
            false
        } else {
            let h = l[3].string()?;
            match &h[..] {
                "hide" => true,
                _ => false,
            }
        };
        Ok(Layer {
            num: num,
            layer: layer,
            layer_type: layer_type,
            hide: hide,
        })
    }
}

impl FromSexp for LayerType {
    fn from_sexp(s: &Sexp) -> Result<LayerType> {
        let x = s.string()?;
        match &x[..] {
            "signal" => Ok(LayerType::Signal),
            "user" => Ok(LayerType::User),
            _ => str_error(format!("unknown layertype {} in {}", x, s)),
        }
    }
}

impl FromSexp for SetupElement {
    fn from_sexp(s: &Sexp) -> Result<SetupElement> {
        let l = s.list()?;
        if l.len() != 2 && l.len() != 3 {
            return str_error(format!("expecting 2 or 3 elements in setup element: {}", s));
        }
        let name = l[0].string()?.clone();
        let value1 = l[1].string()?.clone();
        let value2 = match l.len() {
            3 => Some(l[2].string()?.clone()),
            _ => None,
        };
        Ok(SetupElement {
            name: name,
            value1: value1,
            value2: value2,
        })
    }
}

impl FromSexp for NetClass {
    fn from_sexp(s: &Sexp) -> Result<NetClass> {
        fn parse(e: &Sexp, name: &str) -> Result<f64> {
            let l = e.slice_atom(name)?;
            l[0].f().map_err(From::from)
        }
        let l = s.slice_atom("net_class")?;
        let name = l[0].string()?.clone();
        let desc = l[1].string()?.clone();
        let mut clearance = 0.1524;
        let mut trace_width = 0.2032;
        let mut via_dia = 0.675;
        let mut via_drill = 0.25;
        let mut uvia_dia = 0.508;
        let mut uvia_drill = 0.127;
        let mut diff_pair_gap = None;
        let mut diff_pair_width = None;
        let mut nets = vec![];
        for x in &l[2..] {
            let list_name = x.list_name()?;
            let xn = &list_name[..];
            match xn {
                "add_net" => {
                    let l1 = x.slice_atom("add_net")?;
                    nets.push(l1[0].string()?.clone())
                }
                "clearance" => clearance = parse(x, xn)?,
                "trace_width" => trace_width = parse(x, xn)?,
                "via_dia" => via_dia = parse(x, xn)?,
                "via_drill" => via_drill = parse(x, xn)?,
                "uvia_dia" => uvia_dia = parse(x, xn)?,
                "uvia_drill" => uvia_drill = parse(x, xn)?,
                "diff_pair_gap" => diff_pair_gap = Some(parse(x, xn)?),
                "diff_pair_width" => diff_pair_width = Some(parse(x, xn)?),
                _ => return str_error(format!("unknown net_class field {}", list_name)),
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
            nets: nets,
            trace_width: trace_width,
        };
        Ok(net_class)
    }
}

impl FromSexp for Setup {
    fn from_sexp(s: &Sexp) -> Result<Setup> {
        let mut elements = vec![];
        let mut pcbplotparams = vec![];
        for v in s.slice_atom("setup")? {
            let n = v.list_name().unwrap().clone();
            match &n[..] {
                "pcbplotparams" => {
                    for y in v.slice_atom("pcbplotparams")? {
                        let p_e = from_sexp(y)?;
                        pcbplotparams.push(p_e)
                    }
                }
                _ => {
                    let setup_element = from_sexp(&v)?;
                    elements.push(setup_element)
                }
            }
        }
        let s = Setup {
            elements: elements,
            pcbplotparams: pcbplotparams,
        };
        Ok(s)
    }
}

// for some reason this needs to be in a subfunction or it doesn't work
fn parse_other(e: &Sexp) -> Element {
    let e2 = e.clone();
    debug!("Element::Other: {}", e2);
    Element::Other(e2)
}

impl FromSexp for GrText {
    fn from_sexp(s: &Sexp) -> Result<GrText> {
        let l = s.slice_atom("gr_text")?;
        let value = l[0].string()?.clone();
        let mut layer = footprint::Layer::default();
        let mut tstamp = None;
        let mut at = footprint::At::default();
        let mut effects = footprint::Effects::default();
        for x in &l[1..] {
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
    fn from_sexp(s: &Sexp) -> Result<GrElement> {
        match &(s.list_name()?)[..] {
            "start" => wrap(s, from_sexp, GrElement::Start),
            "end" => wrap(s, from_sexp, GrElement::End),
            "center" => wrap(s, from_sexp, GrElement::Center),
            "angle" => {
                let l2 = s.slice_atom("angle")?;
                Ok(GrElement::Angle(l2[0].f()?))
            }
            "layer" => wrap(s, from_sexp, GrElement::Layer),
            "width" => {
                let l2 = s.slice_atom("width")?;
                Ok(GrElement::Width(l2[0].f()?))
            }
            "size" => {
                let l2 = s.slice_atom("size")?;
                Ok(GrElement::Size(l2[0].f()?))
            }
            "drill" => {
                let l2 = s.slice_atom("drill")?;
                Ok(GrElement::Drill(l2[0].f()?))
            }
            "tstamp" => {
                let l2 = s.slice_atom("tstamp")?;
                let sx = l2[0].string()?.clone();
                Ok(GrElement::TStamp(sx))
            }
            "status" => {
                let l2 = s.slice_atom("status")?;
                let sx = l2[0].string()?.clone();
                Ok(GrElement::Status(sx))
            }
            "net" => {
                let l2 = s.slice_atom("net")?;
                Ok(GrElement::Net(l2[0].i()?))
            },
            "at" => wrap(s, from_sexp, GrElement::At),
            "layers" => wrap(s, from_sexp, GrElement::Layers),
            "effects" => wrap(s, from_sexp, GrElement::Effects),
            x => str_error(format!("unknown element {} in {}", x, s)),
        }
    }
}


impl FromSexp for GrLine {
    fn from_sexp(s: &Sexp) -> Result<GrLine> {
        // println!("GrLine: {}", s);
        let l = s.slice_atom("gr_line")?;
        let mut start = footprint::Xy::new_empty(footprint::XyType::Start);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut angle = 0.0_f64;
        let mut layer = footprint::Layer::default();
        let mut width = 0.0_f64;
        let mut tstamp = None;
        for x in l {
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
    fn from_sexp(s: &Sexp) -> Result<GrArc> {
        let l = s.slice_atom("gr_arc")?;
        let mut start = footprint::Xy::new_empty(footprint::XyType::Start);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut angle = 0.0_f64;
        let mut layer = footprint::Layer::default();
        let mut width = 0.0_f64;
        let mut tstamp = None;
        for x in l {
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
    fn from_sexp(s: &Sexp) -> Result<GrCircle> {
        let l = s.slice_atom("gr_circle")?;
        let mut center = footprint::Xy::new_empty(footprint::XyType::Center);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut layer = footprint::Layer::default();
        let mut width = 0.0_f64;
        let mut tstamp = None;
        for x in l {
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
    fn from_sexp(s: &Sexp) -> Result<Dimension> {
        let l = s.slice_atom_num("dimension", 11)?;
        let name = l[0].string()?.clone();
        let width = {
            let l2 = l[1].slice_atom("width")?;
            l2[0].f()?
        };
        let layer = from_sexp(&l[2])?;
        let (i, tstamp) = match l[3].named_value_string("tstamp") {
            Ok(s) => (4, Some(s.clone())),
            _ => (3, None),
        };
        let text = from_sexp(&l[i])?;
        let feature1 = from_sexp(l[i + 1].named_value("feature1")?)?;
        let feature2 = from_sexp(l[i + 2].named_value("feature2")?)?;
        let crossbar = from_sexp(l[i + 3].named_value("crossbar")?)?;
        let arrow1a = from_sexp(l[i + 4].named_value("arrow1a")?)?;
        let arrow1b = from_sexp(l[i + 5].named_value("arrow1b")?)?;
        let arrow2a = from_sexp(l[i + 6].named_value("arrow2a")?)?;
        let arrow2b = from_sexp(l[i + 7].named_value("arrow2b")?)?;
        Ok(Dimension {
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
    fn from_sexp(s: &Sexp) -> Result<Zone> {
        let mut i = IterAtom::new(s, "zone")?;
        let net:ZoneNet = i.t("zone", "net")?;
        let net_name:ZoneNetName = i.t("zone", "net_name")?;
        let layer = i.t("zone", "layer")?;
        let tstamp:TStamp = i.t("zone", "tstamp")?;
        let hatch = i.t("zone", "hatch")?;
        let priority:Option<Priority> = i.maybe_t();
        let priority = match priority {
            Some(p) => p.0 as u64,
            None => 0_u64,
        };
        let connect_pads = i.t("zone", "connect_pads")?;
        let min_thickness:MinThickness = i.t("zone", "min_thickness")?;
        let keepout = i.maybe_t();
        let fill = i.t("zone", "fill")?;
        let other = i.iter.cloned().collect();
        for x in &other {
            debug!("'zone': not parsing {}", x);
        }
        Ok(Zone {
            net: net.0,
            net_name: net_name.0,
            layer: layer,
            tstamp: tstamp.0,
            hatch: hatch,
            priority: priority,
            connect_pads:connect_pads,
            min_thickness:min_thickness.0,
            keepout: keepout,
            fill: fill,
            other: other,
        })
    }
}

impl FromSexp for Hatch {
    fn from_sexp(s: &Sexp) -> Result<Hatch> {
        let l = s.slice_atom_num("hatch", 2)?;
        let style = l[0].string()?.clone();
        let pitch = l[1].f()?;
        Ok(Hatch { style:style, pitch:pitch })
    }
}

impl FromSexp for ConnectPads {
    fn from_sexp(s: &Sexp) -> Result<ConnectPads> {
        let l = s.slice_atom("connect_pads")?;
        let (connection,clearance) = if l.len() == 1 {
            (None, l[0].named_value_f("clearance")?)
        } else if l.len() == 2 {
            (Some(l[0].string()?.clone()), l[1].named_value_f("clearance")?)
        } else {
            return Err("unknown extra elements in connect_pads".into())
        };
        Ok(ConnectPads { connection:connection, clearance:clearance })
    }
}
impl FromSexp for Keepout {
    fn from_sexp(s: &Sexp) -> Result<Keepout> {
        let l = s.slice_atom_num("keepout", 3)?;
        let tracks = !l[0].named_value_string("tracks")?.starts_with("not");
        let vias = !l[1].named_value_string("vias")?.starts_with("not");
        let copperpour = !l[2].named_value_string("copperpour")?.starts_with("not");
        Ok(Keepout { tracks:tracks, vias:vias, copperpour:copperpour })
    }
}

//  (fill yes (arc_segments 16) (thermal_gap 0.508) (thermal_bridge_width 0.508))
impl FromSexp for Fill {
    fn from_sexp(s: &Sexp) -> Result<Fill> {
        let mut i = IterAtom::new(s, "fill")?;
        let filled = i.maybe_s().is_some();
        let mode:Option<FillMode> = i.maybe_t();
        let arc_segments:FillArcSegments = i.t("fill", "arc_segments")?;
        let thermal_gap:FillThermalGap = i.t("fill", "thermal_gap")?;
        let thermal_bridge_width:FillThermalBridgeWidth = i.t("fill", "thermal_bridge_width")?;
        let smoothing:Option<FillSmoothing> = i.maybe_t();
        let radius:Option<FillRadius> = i.maybe_t();
        let radius = match radius {
            Some(x) => x.0,
            None => 0_f64,
        };
        Ok(Fill {
            filled:filled,
            segment:mode.is_some(),
            arc_segments: arc_segments.0,
            thermal_gap:thermal_gap.0,
            thermal_bridge_width: thermal_bridge_width.0,
            smoothing:smoothing.map(|x| x.0),
            corner_radius:radius,
        })
    }
}

impl FromSexp for Segment {
    fn from_sexp(s: &Sexp) -> Result<Segment> {
        // println!("GrLine: {}", s);
        let l = s.slice_atom("segment")?;
        let mut start = footprint::Xy::new_empty(footprint::XyType::Start);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut layer = footprint::Layer::default();
        let mut width = 0.0_f64;
        let mut tstamp = None;
        let mut net = 0;
        let mut status = None;
        for x in l {
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
    fn from_sexp(s: &Sexp) -> Result<Via> {
        let l = s.slice_atom("via")?;
        let mut at = footprint::At::default();
        let mut size = 0.0_f64;
        let mut drill = 0.0_f64;
        let mut layers = footprint::Layers::default();
        let mut net = 0;
        for x in l {
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
    fn from_sexp(s: &Sexp) -> Result<Layout> {
        let l1 = s.slice_atom("kicad_pcb")?;
        let mut layout = Layout::default();
        for e in l1 {
            match &(e.list_name()?)[..] {
                "version" => layout.version = parse_version(e)?,
                "host" => layout.host = from_sexp(e)?,
                "general" => layout.general = from_sexp(&e)?,
                "page" => layout.page = parse_page(&e)?,
                "layers" => layout.layers = from_sexp(e)?,
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
                "setup" => layout.setup = from_sexp(&e)?,
                _ => {
                    //println!("unimplemented: {}", e);
                    layout.elements.push(parse_other(e))
                }
            }
        }
        Ok(layout)
    }
}
