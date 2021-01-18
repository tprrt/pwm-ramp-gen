.. image:: https://circleci.com/gh/tprrt/pwm-ramp-gen.svg?style=svg&circle-token=8794b4eb585ada86a0521f8c215903faa223de40
    :alt: Circle badge
    :target: https://app.circleci.com/pipelines/github/tprrt/pwm-ramp-gen

.. image:: https://sonarcloud.io/api/project_badges/measure?project=tprrt_pwm-ramp-gen&metric=alert_status
    :alt: Quality Gate Status
    :target: https://sonarcloud.io/dashboard?id=tprrt_pwm-ramp-gen

==================
PWM ramp generator
==================

It is a simple `RIOT-OS`_ application which generate a continuous PWM signal of
1Khz, on the first channel of the first PWM device, and whose duty which
increases (then decreases) every 10ms.

::

    # To pull the submodules' sources
    git submodule sync
    git submodule update --init --recursive

    # To pull the container to cross-compile
    podman pull riot/riotbuild

    # To build in the container
    export BUILD_IN_DOCKER=1
    export DOCKER="podman"
    export DOCKER_IMAGE=riot/riotbuild:latest
    export DOCKER_USER=""
    export DOCKER_RUN_FLAGS="--rm -i -t --security-opt seccomp=unconfined --security-opt label=disable --userns=keep-id"
    export DOCKER_MAKE_ARGS="-j$(nproc)"

    make


----

Use the following command to validate the `circleci`_ pipeline:

::

    podman run --rm --security-opt seccomp=unconfined --security-opt label=disable -v $(pwd):/data circleci/circleci-cli:alpine config validate /data/.circleci/config.yml --token $TOKEN


.. _circleci: https://circleci.com
.. _RIOT-OS: https://github.com/RIOT-OS/RIOT
