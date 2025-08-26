# Copyright (c) 2023-2025 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
import sys
from copy import deepcopy
from unittest.mock import patch

import pytest

from pyavd import get_avd_facts
from pyavd._eos_designs.eos_designs_facts.schema import EosDesignsFacts
from tests.models import MoleculeScenario


@pytest.mark.molecule_scenarios(
    "digital_twin",
    "eos_designs_unit_tests",
    "eos_designs_deprecated_vars",
    "eos_designs-l2ls",
    "eos_designs-mpls-isis-sr-ldp",
    # TODO: "eos_designs-twodc-5stage-clos", # Remove inline jinja
    "evpn_underlay_ebgp_overlay_ebgp",
    "evpn_underlay_isis_overlay_ibgp",
    "evpn_underlay_ospf_overlay_ebgp",
    "evpn_underlay_rfc5549_overlay_ebgp",
    "example-campus-fabric",
    # TODO: "example-cv-pathfinder", # Work around Ansible vault
    "example-dual-dc-l3ls",
    "example-isis-ldp-ipvpn",
    "example-l2ls-fabric",
    "example-single-dc-l3ls",
    "example-single-dc-l3ls-ipv6",
)
@pytest.mark.digital_twin_molecule_scenarios("eos_designs-twodc-5stage-clos", "digital_twin")
def test_get_avd_facts(molecule_scenario: MoleculeScenario) -> None:
    """Test get_avd_facts."""
    molecule_inputs = {host.name: deepcopy(host.hostvars) for host in molecule_scenario.hosts}

    with patch("sys.path", [*sys.path, *molecule_scenario.extra_python_paths]):
        avd_facts = get_avd_facts(molecule_inputs, pool_manager=molecule_scenario.pool_manager, digital_twin=molecule_scenario.digital_twin)

    assert isinstance(avd_facts, dict)
    assert len(avd_facts) == len(molecule_inputs)
    assert avd_facts.keys() == molecule_inputs.keys()
    if avd_facts:
        assert isinstance(next(iter(avd_facts.values())), EosDesignsFacts)
