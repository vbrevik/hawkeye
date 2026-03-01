"""
Hawkeye Embedding Sidecar — bge-m3 via sentence-transformers + FastAPI

OpenAI-compatible /v1/embeddings endpoint serving BAAI/bge-m3 (1024-dim).
Intended to be run via: uv run --python 3.12 --with ... scripts/embed_server.py
"""

import os
import time
import uvicorn
from contextlib import asynccontextmanager
from fastapi import FastAPI
from pydantic import BaseModel

MODEL_NAME = os.environ.get("EMBED_MODEL", "BAAI/bge-m3")

model = None


@asynccontextmanager
async def lifespan(app: FastAPI):
    global model
    from sentence_transformers import SentenceTransformer

    print(f"Loading model: {MODEL_NAME}")
    start = time.time()
    model = SentenceTransformer(MODEL_NAME)
    elapsed = time.time() - start
    dim = model.get_sentence_embedding_dimension()
    print(f"Model loaded in {elapsed:.1f}s — dimension: {dim}")
    yield


app = FastAPI(title="Hawkeye Embedding Sidecar", lifespan=lifespan)


class EmbedRequest(BaseModel):
    model: str = MODEL_NAME
    input: list[str] | str


class EmbedData(BaseModel):
    embedding: list[float]
    index: int
    object: str = "embedding"


class EmbedUsage(BaseModel):
    prompt_tokens: int
    total_tokens: int


class EmbedResponse(BaseModel):
    data: list[EmbedData]
    model: str
    object: str = "list"
    usage: EmbedUsage


@app.post("/v1/embeddings")
async def embed(req: EmbedRequest) -> EmbedResponse:
    texts = [req.input] if isinstance(req.input, str) else req.input
    embeddings = model.encode(texts, normalize_embeddings=True)

    data = [
        EmbedData(embedding=emb.tolist(), index=i)
        for i, emb in enumerate(embeddings)
    ]
    token_count = sum(len(t.split()) for t in texts)

    return EmbedResponse(
        data=data,
        model=req.model,
        usage=EmbedUsage(prompt_tokens=token_count, total_tokens=token_count),
    )


@app.get("/health")
async def health():
    return {"status": "ok", "model": MODEL_NAME}


if __name__ == "__main__":
    port = int(os.environ.get("EMBED_PORT", "7703"))
    uvicorn.run(app, host="0.0.0.0", port=port)
