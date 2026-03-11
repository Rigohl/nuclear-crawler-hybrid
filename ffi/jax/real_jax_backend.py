import json
import sys

import jax
import jax.numpy as jnp


def _read_payload():
    return json.load(sys.stdin)


def _string_matrix(texts, dim=64):
    rows = []
    lengths = []
    for text in texts:
        encoded = text.encode("utf-8", "ignore")
        lengths.append(float(len(encoded)))
        vector = jnp.zeros((dim,), dtype=jnp.float32)
        if encoded:
            indexes = jnp.array([byte % dim for byte in encoded], dtype=jnp.int32)
            updates = jnp.ones((indexes.shape[0],), dtype=jnp.float32)
            vector = vector.at[indexes].add(updates)
            vector = vector / jnp.maximum(jnp.sum(vector), 1.0)
        rows.append(vector)
    matrix = jnp.stack(rows) if rows else jnp.zeros((0, dim), dtype=jnp.float32)
    return matrix, jnp.array(lengths, dtype=jnp.float32)


def process_batch(payload):
    texts = payload["results"]
    _, lengths = _string_matrix(texts)
    scores = jax.nn.sigmoid((lengths - 128.0) / 64.0)
    return jax.device_get(scores).tolist()


def vectorize(payload):
    texts = payload["content"]
    matrix, _ = _string_matrix(texts)
    return jax.device_get(matrix).tolist()


def similarity(payload):
    vectors = jnp.array(payload["vectors"], dtype=jnp.float32)
    norms = jnp.linalg.norm(vectors, axis=1, keepdims=True)
    norms = jnp.where(norms == 0, 1, norms)
    normalized = vectors / norms
    matrix = normalized @ normalized.T
    return jax.device_get(matrix).tolist()


OPERATIONS = {
    "process_batch": process_batch,
    "vectorize": vectorize,
    "similarity": similarity,
}


def main():
    if len(sys.argv) != 2 or sys.argv[1] not in OPERATIONS:
        print("Usage: real_jax_backend.py [process_batch|vectorize|similarity]", file=sys.stderr)
        sys.exit(2)

    payload = _read_payload()
    result = OPERATIONS[sys.argv[1]](payload)
    print(json.dumps(result))


if __name__ == "__main__":
    main()