import os

from fastapi import FastAPI
from fastapi.responses import JSONResponse

app = FastAPI()

PORT = int(os.environ.get('PORT', 10000))


@app.get('/')
def index() -> dict[str, str]:
    return {'message': 'Hello from FastAPI on Partiri!'}


@app.get('/health')
def health() -> JSONResponse:
    return JSONResponse(content={'status': 'ok'})
