from .mongo_emb import PyRdb
from typing import List


class PyRedb:

    def __init__(self, dp: str, tp: str) -> None:
        self.dp = dp
        self.tp = tp
        self.__rust_db = PyRdb(self.dp, self.tp)

    def __enter__(self):
        self.__rust_db = PyRdb(self.dp, self.tp)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        pass

    def write(self, k, v):
        return self.__rust_db.write(k, v)

    def read(self, k):
        return self.__rust_db.read(k)
