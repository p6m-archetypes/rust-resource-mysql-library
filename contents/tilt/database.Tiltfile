# -*- mode: Python -*-

local_resource(
    'database',
    serve_cmd='docker compose up mysql --wait',
    readiness_probe=probe(
        exec=exec_action(['docker', 'compose', 'exec', 'mysql', 'mysqladmin', 'ping', '-h', 'localhost', '-u', 'dev', '-pdev']),
        period_secs=5,
    ),
)
