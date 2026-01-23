class Persistence:

    def __init__(self):
        self._data = set()

    def add_token(self, token):
        self._data.add(token)

    def remove_token(self, token):
        self._data.remove(token)

    def contains_token(self, token):
        return token in self._data


