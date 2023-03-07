#![allow(dead_code)]

use std::{collections::HashMap, ops::Deref, str::FromStr};

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
struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(lat: f64, lon: f64) -> Self {
        GeoPoint(lat, lon)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
struct GeoPolygon(Vec<(f64, f64)>);

impl GeoPolygon {
    fn new(points: Vec<(f64, f64)>) -> Self {
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
struct Map<T: Parse>(HashMap<String, T>);

impl<T: Parse> Map<T> {
    fn new() -> Self {
        Map(HashMap::new())
    }

    fn insert(&mut self, key: String, value: T) {
        self.0.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&T> {
        self.0.get(key)
    }
}

impl<T: Parse> IntoIterator for Map<T> {
    type Item = (String, T);
    type IntoIter = std::collections::hash_map::IntoIter<String, T>;

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

fn main() {
    println!("Hello, world!");
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
            version: Version {
                major: Integer(1),
                minor: Integer(0),
                patch: Integer(0),
            },
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
