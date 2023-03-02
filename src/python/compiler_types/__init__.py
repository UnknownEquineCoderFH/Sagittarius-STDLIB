from __future__ import annotations


from enum import StrEnum, auto
from typing import (
    NamedTuple,
    TypeAlias,
    Protocol,
    TypeVar,
    Any,
    Sequence,
    Iterable,
    MutableMapping,
    Generic,
    Self,
)
from urllib.parse import ParseResult as URI

"""
Globals section / Wraps / Protocols
"""


class Parse(Protocol):
    @classmethod
    def parse(cls, input: str) -> tuple[Self, str]:
        ...


def parse_types(t: tuple[Parse, ...]) -> Iterable[type[Parse]]:
    for val in t:
        yield type(val)


class ParseKey(Protocol):
    @classmethod
    def parse_key(cls) -> str:
        ...


Wrapped = TypeVar("Wrapped")


class Wrap(Generic[Wrapped]):
    def __init__(self, value: Wrapped, /) -> None:
        self.value = value

    def __getattr__(self, name: str) -> Any:
        return getattr(self.value, name)


class String(Wrap[str]):
    def __str__(self) -> str:
        return "String"

    def __repr__(self) -> str:
        return "String"


class Integer(Wrap[int]):
    def __str__(self) -> str:
        return "Integer"

    def __repr__(self) -> str:
        return "Integer"


class Double(Wrap[float]):
    def __str__(self) -> str:
        return "Double"

    def __repr__(self) -> str:
        return "Double"


class Boolean(Wrap[bool]):
    def __str__(self) -> str:
        return "Boolean"

    def __repr__(self) -> str:
        return "Boolean"


class Timestamp(Wrap[str]):
    def __str__(self) -> str:
        return "Timestamp"

    def __repr__(self) -> str:
        return "Timestamp"


class Geolocation(Wrap[str]):
    def __str__(self) -> str:
        return "Geolocation"

    def __repr__(self) -> str:
        return "Geolocation"


Types: TypeAlias = String | Integer | Double | Boolean | Timestamp | Geolocation

InType = TypeVar("InType", bound=Types, contravariant=True)
OutType = TypeVar("OutType", covariant=True)
T = TypeVar("T")


class Vec(Wrap[list[T]]):
    def __str__(self) -> str:
        return f"{self.value}"

    def __repr__(self) -> str:
        return f"Vec([{','.join(repr(val) for val in self.value)}])"


class Map(Wrap[dict[str, T]]):
    def __str__(self) -> str:
        return f"{self.value}"

    def __repr__(self) -> str:
        return f"Map({{{','.join(f'{key}:{repr(val)}' for key, val in self.value.items())}}})"


class Vis(Protocol[InType, OutType]):
    def generate(self, data: InType) -> OutType:
        ...


class Version(NamedTuple):
    major: Integer
    minor: Integer
    patch: Integer

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
    name: String
    version: Version
    scope: Scope

    @classmethod
    def parse_key(cls) -> str:
        return ".service"

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
    props: Map[Types]

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
    sensors: Map[Sensor]

    @classmethod
    def parse_key(cls) -> str:
        return ".data"

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


class TableGraph:
    ...


class ChartGraph:
    ...


class MapGraph:
    ...


class LineGraph:
    ...


class TableVis(NamedTuple):
    def generate(self, data: Types) -> TableGraph:
        """
        TODO: DEFINE TYPES AND IMPLEMENTATION
        It should take some input data and return a graph type
        """
        ...

    def __str__(self) -> str:
        return "TableVis"

    def __repr__(self) -> str:
        return "TableVis"


class ChartVis(NamedTuple):
    def generate(self, data: Types) -> ChartGraph:
        """
        TODO: DEFINE TYPES AND IMPLEMENTATION
        It should take some input data and return a graph type
        """
        ...

    def __str__(self) -> str:
        return "ChartVis"

    def __repr__(self) -> str:
        return "ChartVis"


class MapVis(NamedTuple):
    def generate(self, data: Types) -> MapGraph:
        """
        TODO: DEFINE TYPES AND IMPLEMENTATION
        It should take some input data and return a graph type
        """
        ...

    def __str__(self) -> str:
        return "MapVis"

    def __repr__(self) -> str:
        return "MapVis"


class LineVis(NamedTuple):
    def generate(self, data: Types) -> LineGraph:
        """
        TODO: DEFINE TYPES AND IMPLEMENTATION
        It should take some input data and return a graph type
        """
        ...

    def __str__(self) -> str:
        return "LineVis"

    def __repr__(self) -> str:
        return "LineVis"


class Visualizations(NamedTuple):
    type: type[Vis[Types, Types]]
    format: Map[type[Types]]

    def __str__(self) -> str:
        return f"{self.type} ({self.format})"

    def __repr__(self) -> str:
        return f"Visualization({self.type}, {self.format})"


class Application(NamedTuple):
    type: AppType
    layout: AppLayout
    graphs: Map[Visualizations]

    @classmethod
    def parse_key(cls) -> str:
        return ".application"

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
    port: Integer | None
    type: DeploymentType
    credentials: Map[String] | None

    def __str__(self) -> str:
        return f"{self.uri}:{self.port} ({self.type})"

    def __repr__(self) -> str:
        return (
            f"DeploymentEnv({self.uri}, {self.port}, {self.type}, {self.credentials})"
        )


class Deployment(NamedTuple):
    envs: Map[DeploymentEnv]

    @classmethod
    def parse_key(cls) -> str:
        return ".deployment"

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
