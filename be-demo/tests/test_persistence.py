import unittest
from backend.persistence import Persistence

class TestPersistence(unittest.TestCase):

    def setUp(self):
        self.persistence = Persistence()

    def test_add_token(self):
        self.persistence.add_token("token1")
        self.assertTrue(self.persistence.contains_token("token1"))

    def test_remove_token(self):
        self.persistence.add_token("token1")
        self.persistence.remove_token("token1")
        self.assertFalse(self.persistence.contains_token("token1"))

    def test_contains_token(self):
        self.persistence.add_token("token1")
        self.assertTrue(self.persistence.contains_token("token1"))
        self.assertFalse(self.persistence.contains_token("token2"))

    def test_remove_non_existent_token(self):
        self.assertFalse(self.persistence.remove_token("non_existent_token"))

if __name__ == '__main__':
    unittest.main()