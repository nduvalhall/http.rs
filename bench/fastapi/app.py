import uvicorn
import fastapi


app = fastapi.FastAPI()


@app.get("/")
def index(): ...


if __name__ == "__main__":
    uvicorn.run(host="localhost", port=8082, app=app)
