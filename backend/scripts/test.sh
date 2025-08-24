#!/bin/bash

# Smart Home Backend Test Script
# This script provides various testing options for the backend

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Help function
show_help() {
    echo "Smart Home Backend Test Runner"
    echo ""
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Options:"
    echo "  unit         Run only unit tests (fast)"
    echo "  integration  Run integration tests (requires database)"
    echo "  all          Run all tests"
    echo "  coverage     Run tests with coverage report"
    echo "  clippy       Run clippy linting"
    echo "  format       Check code formatting"
    echo "  check        Run basic checks (clippy + format)"
    echo "  ci           Run full CI pipeline (check + unit tests)"
    echo "  help         Show this help message"
    echo ""
    echo "Environment Variables:"
    echo "  TEST_DATABASE_URL  Database URL for integration tests"
    echo "  RUST_LOG          Set logging level (default: warn)"
    echo ""
    echo "Examples:"
    echo "  $0 unit                    # Run unit tests only"
    echo "  $0 integration             # Run integration tests"
    echo "  $0 coverage                # Generate coverage report"
    echo "  TEST_DATABASE_URL=postgres://user:pass@localhost/test_db $0 integration"
    echo ""
}

# Check if cargo is available
check_cargo() {
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust and Cargo."
        exit 1
    fi
}

# Run unit tests
run_unit_tests() {
    print_info "Running unit tests..."
    export RUST_LOG=${RUST_LOG:-warn}

    if cargo test --workspace --exclude integration_tests; then
        print_success "Unit tests passed!"
    else
        print_error "Unit tests failed!"
        exit 1
    fi
}

# Run integration tests
run_integration_tests() {
    print_info "Running integration tests..."

    setup_test_db # Ensure test database is set up and migrated

    # Check if database URL is set
    if [ -z "$TEST_DATABASE_URL" ]; then
        print_warning "TEST_DATABASE_URL not set. Using default test database configuration."
        export DATABASE_URL="postgres://smart_home_user:1234@127.0.0.1:5432/smart_home_test"
    else
        export DATABASE_URL="$TEST_DATABASE_URL"
    fi

    export RUST_LOG=${RUST_LOG:-info}

    print_info "Using database: $DATABASE_URL"

    if cargo test --workspace; then
        print_success "Integration tests passed!"
    else
        print_error "Integration tests failed!"
        print_info "Make sure your test database is properly configured and running."
        exit 1
    fi
}

# Run all tests
run_all_tests() {
    print_info "Running all tests..."
    setup_test_db
    if cargo test --workspace; then
        print_success "All tests passed!"
    else
        print_error "All tests failed!"
        exit 1
    fi
}

# Run tests with coverage
run_coverage() {
    print_info "Running tests with coverage..."

    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_warning "cargo-tarpaulin not found. Installing..."
        cargo install cargo-tarpaulin
    fi

    export RUST_LOG=${RUST_LOG:-warn}

    if cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Html --output-dir coverage/; then
        print_success "Coverage report generated in coverage/ directory"
        print_info "Open coverage/tarpaulin-report.html in your browser to view the report"
    else
        print_error "Coverage generation failed!"
        exit 1
    fi
}

# Run clippy linting
run_clippy() {
    print_info "Running clippy linting..."

    if cargo clippy --all-targets --all-features -- -D warnings; then
        print_success "Clippy checks passed!"
    else
        print_error "Clippy found issues!"
        exit 1
    fi
}

# Check code formatting
check_format() {
    print_info "Checking code formatting..."

    if cargo fmt --all -- --check; then
        print_success "Code formatting is correct!"
    else
        print_error "Code formatting issues found!"
        print_info "Run 'cargo fmt' to fix formatting issues."
        exit 1
    fi
}

# Run basic checks
run_checks() {
    print_info "Running basic checks..."
    check_format
    run_clippy
    print_success "All checks passed!"
}

# Run CI pipeline
run_ci() {
    print_info "Running CI pipeline..."
    run_checks
    run_unit_tests
    print_success "CI pipeline completed successfully!"
}

# Setup test database (helper function)
setup_test_db() {
    print_info "Setting up test database..."

    # Default test database configuration
    DB_USER=${DB_USER:-"smart_home_user"}
    DB_PASS=${DB_PASS:-"1234"}
    DB_HOST=${DB_HOST:-"127.0.0.1"}
    DB_PORT=${DB_PORT:-"5432"}
    DB_NAME=${DB_NAME:-"smart_home_test"}

    print_info "Creating test database if it doesn't exist..."

    # Try to create database (will fail if it already exists, which is fine)
    PGPASSWORD=$DB_PASS createdb -h $DB_HOST -p $DB_PORT -U $DB_USER $DB_NAME 2>/dev/null || true

    print_info "Running migrations on test database..."
    export DATABASE_URL="postgres://$DB_USER:$DB_PASS@$DB_HOST:$DB_PORT/$DB_NAME"

    if command -v sqlx &> /dev/null; then
        if sqlx migrate run; then
            print_success "Migrations applied successfully!"
            print_success "Test database setup completed!"
        else
            print_warning "Migrations failed. Recreating database..."

            PGPASSWORD=$DB_PASS dropdb -h $DB_HOST -p $DB_PORT -U $DB_USER $DB_NAME --if-exists
            PGPASSWORD=$DB_PASS createdb -h $DB_HOST -p $DB_PORT -U $DB_USER $DB_NAME

            if sqlx migrate run; then
                print_success "Migrations applied successfully after database recreation!"
                print_success "Test database setup completed!"
            else
                print_error "Migrations failed even after database recreation!"
                print_error "Please check your migration files for errors."
                return 1
            fi
        fi
    else
        print_warning "sqlx-cli not found. Please run migrations manually."
        print_info "Install with: cargo install sqlx-cli"
        return 1
    fi
}

# Main script logic
main() {
    check_cargo

    case "${1:-help}" in
        "unit")
            run_unit_tests
            ;;
        "integration")
            run_integration_tests
            ;;
        "all")
            run_all_tests
            ;;
        "coverage")
            run_coverage
            ;;
        "clippy")
            run_clippy
            ;;
        "format")
            check_format
            ;;
        "check")
            run_checks
            ;;
        "ci")
            run_ci
            ;;
        "setup-db")
            setup_test_db
            ;;
        "help"|"--help"|"-h")
            show_help
            ;;
        *)
            print_error "Unknown option: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
