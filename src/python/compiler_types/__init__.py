from __future__ import annotations


from enum import StrEnum, auto
from typing import NamedTuple, TypeAlias, Protocol, Self
from urllib.parse import ParseResult as URI

"""
Globals section / Primitives / Protocols
"""


class ParseKey(Protocol):
    @classmethod
    def parse_key(cls) -> str:
        ...


class Primitive:
    pass


class String(Primitive):
    def __str__(self) -> str:
        return "String"

    def __repr__(self) -> str:
        return "String"


class Integer(Primitive):
    def __str__(self) -> str:
        return "Integer"

    def __repr__(self) -> str:
        return "Integer"


class Double(Primitive):
    def __str__(self) -> str:
        return "Double"

    def __repr__(self) -> str:
        return "Double"


class Boolean(Primitive):
    def __str__(self) -> str:
        return "Boolean"

    def __repr__(self) -> str:
        return "Boolean"


class Timestamp(Primitive):
    def __str__(self) -> str:
        return "Timestamp"

    def __repr__(self) -> str:
        return "Timestamp"


class Geolocation(Primitive):
    def __str__(self) -> str:
        return "Geolocation"

    def __repr__(self) -> str:
        return "Geolocation"


Types: TypeAlias = String | Integer | Double | Boolean | Timestamp | Geolocation


class Version(NamedTuple):
    major: int
    minor: int
    patch: int

    def __str__(self) -> str:
        return f"{self.major}.{self.minor}.{self.patch}"

    def __repr__(self) -> str:
        return f"Version({self.major}, {self.minor}, {self.patch})"


"""
Service preamble section
"""


class Scope(StrEnum):
    Service = auto()
    Industry = auto()
    Manifacturing = auto()
    Education = auto()
    Healthcare = auto()
    SocialPrograms = auto()
    Government = auto()
    Energy = auto()
    Water = auto()
    Environment = auto()
    Transportation = auto()
    Communication = auto()
    PublicSafety = auto()
    UrbanPlanning = auto()
    Infrastructure = auto()


class Service(NamedTuple):
    name: str
    version: Version
    scope: Scope

    @classmethod
    def parse_key(cls) -> str:
        return "service"

    def __str__(self) -> str:
        return f"{self.name} v{self.version} ({self.scope})"

    def __repr__(self) -> str:
        return f"Service({self.name}, {self.version}, {self.scope})"


"""
Data section
"""


class Provider(StrEnum):
    Fiware = auto()
    Dataskop = auto()


class SensorFormat(NamedTuple):
    props: dict[str, Types]

    def __str__(self) -> str:
        return f"SensorFormat({self.props})"

    def __repr__(self) -> str:
        return f"SensorFormat({self.props})"


class SensorType(StrEnum):
    SmartMeter = auto()


class Sensor(NamedTuple):
    type: SensorType
    provider: Provider
    uri: URI
    format: SensorFormat

    def __str__(self) -> str:
        return f"{self.type} ({self.provider}, {self.uri})"

    def __repr__(self) -> str:
        return f"Sensor({self.type}, {self.provider}, {self.uri}, {self.format})"


class SensorData(NamedTuple):
    sensors: dict[str, Sensor]

    @classmethod
    def parse_key(cls) -> str:
        return "data"

    def __str__(self) -> str:
        return f"SensorData({self.sensors})"

    def __repr__(self) -> str:
        return f"SensorData({self.sensors})"


"""
Application section
"""


class AppType(StrEnum):
    WebApp = auto()
    MobileApp = auto()
    DesktopApp = auto()
    IoTApp = auto()


class AppLayout(StrEnum):
    SinglePage = auto()
    MultiPage = auto()
    MultiWindow = auto()


class AuthenticationRoles(StrEnum):
    SuperUser = auto()
    Admin = auto()
    User = auto()
    Guest = auto()


class Authentication(NamedTuple):
    name: str
    role: AuthenticationRoles
    default: bool = False

    def __str__(self) -> str:
        return f"{self.name} ({self.role})"

    def __repr__(self) -> str:
        return f"Authentication({self.name}, {self.role}, {self.default})"


class TableVis(NamedTuple):
    def __str__(self) -> str:
        return "TableVis"

    def __repr__(self) -> str:
        return "TableVis"


class ChartVis(NamedTuple):
    def __str__(self) -> str:
        return "ChartVis"

    def __repr__(self) -> str:
        return "ChartVis"


class MapVis(NamedTuple):
    def __str__(self) -> str:
        return "MapVis"

    def __repr__(self) -> str:
        return "MapVis"


class LineVis(NamedTuple):
    def __str__(self) -> str:
        return "LineVis"

    def __repr__(self) -> str:
        return "LineVis"


VisualizationType: TypeAlias = TableVis | ChartVis | MapVis | LineVis


class Visualization(NamedTuple):
    type: VisualizationType
    format: dict[str, Types]

    def __str__(self) -> str:
        return f"{self.type} ({self.format})"

    def __repr__(self) -> str:
        return f"Visualization({self.type}, {self.format})"


class Application(NamedTuple):
    type: AppType
    layout: AppLayout
    graphs: dict[str, Visualization]

    @classmethod
    def parse_key(cls) -> str:
        return "application"

    def __str__(self) -> str:
        return f"Application({self.type}, {self.layout}, {self.graphs})"

    def __repr__(self) -> str:
        return f"Application({self.type}, {self.layout}, {self.graphs})"


"""
Deployment section
"""


class DeploymentType(StrEnum):
    Docker = auto()
    Kubernetes = auto()
    DockerCompose = auto()
    Helm = auto()
    Ansible = auto()
    Terraform = auto()
    CloudFormation = auto()
    Serverless = auto()


class DeploymentEnv(NamedTuple):
    uri: URI
    port: int | None
    type: DeploymentType
    credentials: dict[str, str] | None

    def __str__(self) -> str:
        return f"{self.uri}:{self.port} ({self.type})"

    def __repr__(self) -> str:
        return (
            f"DeploymentEnv({self.uri}, {self.port}, {self.type}, {self.credentials})"
        )


class Deployment(NamedTuple):
    envs: dict[str, DeploymentEnv]

    @classmethod
    def parse_key(cls) -> str:
        return "deployment"

    def __str__(self) -> str:
        return f"Deployment({self.envs})"

    def __repr__(self) -> str:
        return f"Deployment({self.envs})"


"""
File section
"""


class SSDL(NamedTuple):
    service: Service
    data: SensorData
    application: Application
    deployment: Deployment

    @classmethod
    def parse_key(cls) -> str:
        return "ssdl"

    def __str__(self) -> str:
        return (
            f"SSDL({self.service}, {self.data}, {self.application}, {self.deployment})"
        )

    def __repr__(self) -> str:
        return (
            f"SSDL({self.service}, {self.data}, {self.application}, {self.deployment})"
        )
