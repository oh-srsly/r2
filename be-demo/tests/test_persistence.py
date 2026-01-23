import pytest
from backend.persistence import Persistence

def test_add_token():
    p = Persistence()
    p.add_token("token1")
    assert p.contains_token("token1")

def test_remove_token():
    p = Persistence()
    p.add_token("token1")
    p.remove_token("token1")
    assert not p.contains_token("token1")

def test_contains_token():
    p = Persistence()
    p.add_token("token1")
    assert p.contains_token("token1")
    assert not p.contains_token("token2")

def test_remove_non_existent_token():
    p = Persistence()
    with pytest.raises(KeyError):
        p.remove_token("non_existent_token")
