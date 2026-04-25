import uvicorn
import fastapi


app = fastapi.FastAPI()


@app.get("/")
def index():
    pass


if __name__ == "__main__":
    uvicorn.run(host="localhost", port=8082, app=app)


class A:
    def __init__(self) -> None:
        self.a = 1


class B(A):
    def __init__(self) -> None:
        super().__init__()
        self.b = 2


class C(A):
    def __init__(self) -> None:
        super().__init__()
        self.c = 2


def get_a(c: A):
    c.a


def get_a_for_b(c: B):
    c.a


def get_a_for_c(c: C):
    c.a


get_a(B())
get_a(C())
