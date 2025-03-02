# coding: utf-8

"""
    kraken

    The core component of kraken-project

    The version of the OpenAPI document: 0.1.0
    Contact: git@omikron.dev
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json


from typing import Any, ClassVar, Dict, List
from pydantic import BaseModel
from pydantic import Field
from kraken_sdk.models.simple_domain import SimpleDomain
from kraken_sdk.models.simple_port import SimplePort
from kraken_sdk.models.simple_service import SimpleService
try:
    from typing import Self
except ImportError:
    from typing_extensions import Self

class HostRelations(BaseModel):
    """
    A host's direct relations
    """ # noqa: E501
    ports: List[SimplePort] = Field(description="This host's ports")
    services: List[SimpleService] = Field(description="This host's services")
    direct_domains: List[SimpleDomain] = Field(description="Domains pointing to this host via a direct `A` or `AAAA` record")
    indirect_domains: List[SimpleDomain] = Field(description="Domains pointing to this host via a `CNAME` record which eventually resolves to the host")
    __properties: ClassVar[List[str]] = ["ports", "services", "direct_domains", "indirect_domains"]

    model_config = {
        "populate_by_name": True,
        "validate_assignment": True
    }


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of HostRelations from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        _dict = self.model_dump(
            by_alias=True,
            exclude={
            },
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of each item in ports (list)
        _items = []
        if self.ports:
            for _item in self.ports:
                if _item:
                    _items.append(_item.to_dict())
            _dict['ports'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in services (list)
        _items = []
        if self.services:
            for _item in self.services:
                if _item:
                    _items.append(_item.to_dict())
            _dict['services'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in direct_domains (list)
        _items = []
        if self.direct_domains:
            for _item in self.direct_domains:
                if _item:
                    _items.append(_item.to_dict())
            _dict['direct_domains'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in indirect_domains (list)
        _items = []
        if self.indirect_domains:
            for _item in self.indirect_domains:
                if _item:
                    _items.append(_item.to_dict())
            _dict['indirect_domains'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Dict) -> Self:
        """Create an instance of HostRelations from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "ports": [SimplePort.from_dict(_item) for _item in obj.get("ports")] if obj.get("ports") is not None else None,
            "services": [SimpleService.from_dict(_item) for _item in obj.get("services")] if obj.get("services") is not None else None,
            "direct_domains": [SimpleDomain.from_dict(_item) for _item in obj.get("direct_domains")] if obj.get("direct_domains") is not None else None,
            "indirect_domains": [SimpleDomain.from_dict(_item) for _item in obj.get("indirect_domains")] if obj.get("indirect_domains") is not None else None
        })
        return _obj


