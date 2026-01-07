============
Contributing
============

Thank you for considering contributing to the PWM ramp generator project!

Getting Started
===============

1. Fork the repository on GitHub
2. Clone your fork locally:

.. code-block:: bash

    git clone https://github.com/YOUR_USERNAME/pwm-ramp-gen.git
    cd pwm-ramp-gen

3. Pull the submodules' sources:

.. code-block:: bash

    git submodule sync
    git submodule update --init --recursive

Development Environment
=======================

This project uses RIOT-OS and requires a container for cross-compilation:

.. code-block:: bash

    # Pull the container to cross-compile
    podman pull riot/riotbuild

    # Configure the build environment
    export BUILD_IN_DOCKER=1
    export DOCKER="podman"
    export DOCKER_IMAGE=riot/riotbuild:latest
    export DOCKER_USER=""
    export DOCKER_RUN_FLAGS="--rm -i -t --security-opt seccomp=unconfined --security-opt label=disable --userns=keep-id"
    export DOCKER_MAKE_ARGS="-j$(nproc)"

    # Build the project
    make

How to Contribute
=================

Reporting Bugs
--------------

If you find a bug, please open an issue using the bug report template. Include:

- A clear description of the bug
- Steps to reproduce the behavior
- Expected behavior
- Your environment (OS, device, version, etc.)
- Any relevant screenshots or logs

Suggesting Features
-------------------

Feature requests are welcome! Please open an issue using the feature request template and describe:

- The problem you're trying to solve
- Your proposed solution
- Any alternative solutions you've considered

Submitting Changes
------------------

1. Create a new branch for your changes:

.. code-block:: bash

    git checkout -b feature/your-feature-name

2. Make your changes and ensure they follow the project's coding standards

3. Test your changes thoroughly

4. Commit your changes with clear, descriptive commit messages

5. Push to your fork:

.. code-block:: bash

    git push origin feature/your-feature-name

6. Submit a pull request to the main repository

Code Quality
============

This project uses continuous integration with CircleCI and SonarCloud for quality checks.

Before submitting your changes, please:

1. Ensure your code builds successfully
2. Run any available tests
3. Validate the CircleCI pipeline locally:

.. code-block:: bash

    podman run --rm --security-opt seccomp=unconfined --security-opt label=disable -v $(pwd):/data circleci/circleci-cli:alpine config validate /data/.circleci/config.yml --token $TOKEN

Questions?
==========

If you have questions about contributing, feel free to open an issue for discussion.

License
=======

By contributing, you agree that your contributions will be licensed under the same license as the project.
