#![allow(dead_code)]

use map_macro::map;
use std::{collections::HashMap, ops::Deref, str::FromStr, vec};

/*
 * Primitives
 */
trait PrimitiveParse: Sized {
    fn from_stream(stream: &str) -> Result<Self, anyhow::Error>;
}

trait Parse: Sized {
    fn parse(stream: &str) -> Result<(&str, Self), anyhow::Error>;
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
struct Integer(u32);

impl PrimitiveParse for Integer {
    fn from_stream(stream: &str) -> Result<Self, anyhow::Error> {
        Ok(Integer(stream.parse()?))
    }
}

impl Deref for Integer {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Integer(value)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
struct Float(f32);

impl PrimitiveParse for Float {
    fn from_stream(stream: &str) -> Result<Self, anyhow::Error> {
        Ok(Float(stream.parse()?))
    }
}

impl Deref for Float {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(value)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
struct Number(f64);

impl Deref for Number {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PrimitiveParse for Number {
    fn from_stream(stream: &str) -> Result<Self, anyhow::Error> {
        Ok(Number(stream.parse()?))
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number(value)
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
struct Text(String);

impl Deref for Text {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PrimitiveParse for Text {
    fn from_stream(stream: &str) -> Result<Self, anyhow::Error> {
        Ok(Text(stream.to_string()))
    }
}

impl Parse for Text {
    fn parse(_stream: &str) -> Result<(&str, Self), anyhow::Error> {
        todo!()
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text(value)
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text(value.to_string())
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
struct Boolean(bool);

impl Deref for Boolean {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PrimitiveParse for Boolean {
    fn from_stream(stream: &str) -> Result<Self, anyhow::Error> {
        Ok(Boolean(stream.parse()?))
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean(value)
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct DateTime(chrono::NaiveDateTime);

impl DateTime {
    fn new(value: chrono::NaiveDateTime) -> Self {
        DateTime(value)
    }
}

impl Deref for DateTime {
    type Target = chrono::NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PrimitiveParse for DateTime {
    fn from_stream(stream: &str) -> Result<Self, anyhow::Error> {
        match chrono::NaiveDateTime::from_str(stream) {
            Ok(value) => Ok(DateTime(value)),
            Err(message) => Err(anyhow::anyhow!(message)),
        }
    }
}

impl From<chrono::NaiveDateTime> for DateTime {
    fn from(value: chrono::NaiveDateTime) -> Self {
        DateTime(value)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Uri(url::Url);

impl Uri {
    fn new(value: url::Url) -> Self {
        Uri(value)
    }
}

impl Default for Uri {
    fn default() -> Self {
        Uri(url::Url::parse("http://localhost").unwrap())
    }
}

impl Deref for Uri {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PrimitiveParse for Uri {
    fn from_stream(stream: &str) -> Result<Self, anyhow::Error> {
        match url::Url::parse(stream) {
            Ok(value) => Ok(Uri(value)),
            Err(message) => Err(anyhow::anyhow!(message)),
        }
    }
}

impl From<url::Url> for Uri {
    fn from(value: url::Url) -> Self {
        Uri(value)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
struct GeoPoint(Float, Float);

impl GeoPoint {
    fn new(lat: impl Into<Float>, lon: impl Into<Float>) -> Self {
        GeoPoint(lat.into(), lon.into())
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
struct GeoPolygon(Vec<(Float, Float)>);

impl GeoPolygon {
    fn new(points: Vec<(Float, Float)>) -> Self {
        GeoPolygon(points)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
enum Geolocation {
    Point(GeoPoint),
    Polygon(GeoPolygon),
}

impl From<GeoPoint> for Geolocation {
    fn from(value: GeoPoint) -> Self {
        Geolocation::Point(value)
    }
}

impl From<GeoPolygon> for Geolocation {
    fn from(value: GeoPolygon) -> Self {
        Geolocation::Polygon(value)
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
struct Array<T: Parse>(Vec<T>);

impl<T: Parse> Array<T> {
    fn new(value: Vec<T>) -> Self {
        Array(value)
    }

    fn push(&mut self, value: T) {
        self.0.push(value);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }
}

impl<T: Parse> IntoIterator for Array<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct Map<T: Parse>(HashMap<Text, T>);

impl<T: Parse> Map<T> {
    fn new() -> Self {
        Map(HashMap::new())
    }

    fn insert(&mut self, key: impl Into<Text>, value: T) -> &mut Self {
        self.0.insert(key.into(), value);
        self
    }

    fn get(&self, key: impl Into<Text>) -> Option<&T> {
        self.0.get(&key.into())
    }
}

impl<T: Parse> IntoIterator for Map<T> {
    type Item = (Text, T);
    type IntoIter = std::collections::hash_map::IntoIter<Text, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/**
    Service
**/

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::EnumString,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
enum Scope {
    #[default]
    Service,
    Industry,
    Manifacturing,
    Education,
    Healthcare,
    SocialPrograms,
    Government,
    Energy,
    Water,
    Environment,
    Transportation,
    Communication,
    PublicSafety,
    UrbanPlanning,
    Infrastructure,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct Version {
    major: Integer,
    minor: Integer,
    patch: Integer,
}

impl Version {
    fn new(
        major: impl Into<Integer>,
        minor: impl Into<Integer>,
        patch: impl Into<Integer>,
    ) -> Self {
        Version {
            major: major.into(),
            minor: minor.into(),
            patch: patch.into(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct Service {
    version: Version,
    name: Text,
    scope: Scope,
}

impl Parse for Service {
    fn parse(_stream: &str) -> Result<(&str, Self), anyhow::Error> {
        todo!()
    }
}

/**
    Sensor Data
**/

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::EnumString,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
enum Provider {
    Fiware,
    Dataskop,
    Fotec,
    #[default]
    Unknown,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::EnumString,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
enum SourceType {
    Sensor,
    Actuator,
    Device,
    Application,
    Person,
    Vehicle,
    Animal,
    Robot,
    #[default]
    Other,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Query {
    r#type: Text,
    select: Array<Text>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Measurement {
    name: Text,
    provider: Provider,
    r#type: SourceType,
    uri: Uri,
    query: Query,
}

impl Parse for Measurement {
    fn parse(_stream: &str) -> Result<(&str, Self), anyhow::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct DataSources {
    measurements: Map<Measurement>,
}

impl DataSources {
    fn add_measurement(&mut self, measurement: Measurement) -> &mut Self {
        self.measurements
            .insert(measurement.name.clone(), measurement);
        self
    }
}

/*
   Application Data
*/

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::EnumString,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
enum RoleHierarchy {
    #[default]
    User,
    Superuser,
    Admin,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Role {
    name: Text,
    hierarchy: RoleHierarchy,
}

#[derive(Debug, Clone, Default, strum::EnumString, serde::Serialize, serde::Deserialize)]
enum Roles {
    #[default]
    User,
    Superuser,
    Admin,
    Custom(Role),
}

impl Parse for Roles {
    fn parse(_stream: &str) -> Result<(&str, Self), anyhow::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Default, strum::EnumString, serde::Serialize, serde::Deserialize)]
enum VisType {
    Line,
    Bar,
    Pie,
    Table,
    Map,
    #[default]
    Other,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Vis {
    name: Text,
    r#type: VisType,
    source: Text,
    data: Array<Text>,
    extra: Option<Map<Text>>,
}

impl Parse for Vis {
    fn parse(_stream: &str) -> Result<(&str, Self), anyhow::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Default, strum::EnumString, serde::Serialize, serde::Deserialize)]
enum AppType {
    #[default]
    Web,
    Mobile,
    Desktop,
}

#[derive(Debug, Clone, Default, strum::EnumString, serde::Serialize, serde::Deserialize)]
enum LayoutType {
    #[default]
    SinglePage,
    Pwa,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Application {
    r#type: AppType,
    layout: LayoutType,
    roles: Array<Roles>,
    visualizations: Map<Vis>,
}

impl Application {
    fn add_visualization(&mut self, vis: Vis) -> &mut Self {
        self.visualizations.insert(vis.name.clone(), vis);
        self
    }
}

/*
   Deployment Data
*/

#[derive(Debug, Clone, Default, strum::EnumString, serde::Serialize, serde::Deserialize)]
enum DeploymentType {
    #[default]
    Docker,
    Kubernetes,
    Swarm,
    Mesos,
    Nomad,
    Other,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct DeploymentEnv {
    name: Text,
    uri: Uri,
    port: Integer,
    r#type: DeploymentType,
}

impl Parse for DeploymentEnv {
    fn parse(_stream: &str) -> Result<(&str, Self), anyhow::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Deployment {
    env: Map<DeploymentEnv>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct SmartService {
    service: Service,
    data_sources: DataSources,
    application: Application,
    deployment: Deployment,
}

/*
   Parser
*/

struct LineInfo {
    content: &'static str,
    number: u16,
    indentation: u8,
}

struct Parser {
    indentation: u8,
    source: &'static str,
    service: Option<Service>,
    data_sources: Option<DataSources>,
    application: Option<Application>,
    deployment: Option<Deployment>,
}

impl Parser {
    fn new(source: &'static str) -> Self {
        Self {
            indentation: 0,
            source,
            service: None,
            data_sources: None,
            application: None,
            deployment: None,
        }
    }

    fn smart_service(self) -> Result<SmartService, anyhow::Error> {
        Ok(SmartService {
            service: self
                .service
                .ok_or_else(|| anyhow::anyhow!("Service not found"))?,
            data_sources: self
                .data_sources
                .ok_or_else(|| anyhow::anyhow!("Data Sources not found"))?,
            application: self
                .application
                .ok_or_else(|| anyhow::anyhow!("Application not found"))?,
            deployment: self
                .deployment
                .ok_or_else(|| anyhow::anyhow!("Deployment not found"))?,
        })
    }
}

fn main() {
    let _service = Service {
        name: "Air Quality Madrid".into(),
        version: Version {
            major: 1.into(),
            minor: 0.into(),
            patch: 0.into(),
        },
        scope: Scope::Environment,
    };

    let mut _ds = DataSources::default();

    _ds.add_measurement(Measurement {
        name: "Measurements".into(),
        provider: Provider::Fiware,
        r#type: SourceType::Sensor,
        uri: Uri::from_stream("https://data.iiss.at/dataskop/fiwarenosec").unwrap_or_default(),
        query: Query {
            r#type: "AirQualityObserved".into(),
            select: Array(vec![
                "location".into(),
                "Nox".into(),
                "O3".into(),
                "dateObserved".into(),
            ]),
        },
    });

    let mut _app = Application {
        r#type: AppType::Web,
        layout: LayoutType::SinglePage,
        roles: Array(vec![Roles::User, Roles::Superuser, Roles::Admin]),
        ..Default::default()
    };

    _app.add_visualization(Vis {
        name: "Air Quality Visualization".into(),
        r#type: VisType::Map,
        source: "Measurements".into(),
        data: Array(vec![
            "location".into(),
            "address".into(),
            "NOx".into(),
            "O3".into(),
        ]),
        extra: Some(Map(map! {
            "area".into() => "Madrid".into(),
        })),
    });

    let _deployment = Deployment {
        env: Map(map! {
            "local".into() => DeploymentEnv {
                name: "local".into(),
                uri: Uri::from_stream("http://localhost/test").unwrap_or_default(),
                port: 50055.into(),
                r#type: DeploymentType::Docker,
            }
        }),
    };

    let _ss = SmartService {
        service: _service,
        data_sources: _ds.to_owned(),
        application: _app.to_owned(),
        deployment: _deployment,
    };

    let _sr = serde_json::to_string(&_ss).unwrap_or_default();

    println!("{_sr}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_service() {
        let raw = r#"
        service:
            name is Air Quality Madrid
            version is 1.0.0
            scope is Environment
        "#;

        let Ok((_rest, service)) = Service::parse(raw) else { panic!("Could not parse") };

        let expected = Service {
            name: Text("Air Quality Madrid".into()),
            version: Version::new(1, 0, 0),
            scope: Scope::Environment,
        };

        assert_eq!(expected, service);
    }

    #[test]
    fn parse_data_sources() {
        println!("Data Sources");
    }

    #[test]
    fn parse_application() {
        println!("Application");
    }

    #[test]
    fn parse_deployment() {
        println!("Deployment");
    }
}
