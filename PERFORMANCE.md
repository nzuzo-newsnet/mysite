# Performance Optimization Guide

## Current Bottlenecks Identified

### 1. WASM Initialization Delay (~53 seconds in dev mode)
**Problem**: The WASM bundle takes a very long time to download, compile, and initialize before making any server requests.

**Root Causes**:
- Development mode WASM is unoptimized and large
- No code splitting or lazy loading
- All dependencies loaded upfront
- Debug symbols included

### 2. Resource Loading Strategy
**Problem**: All resources wait for WASM to fully load before starting data fetches.

**Impact**:
- Server functions called 50+ seconds after page load
- GitHub API calls blocked until WASM ready
- No content visible during WASM load

### 3. Client-Side GitHub API Calls
**Problem**: GitHub API fetches happen entirely client-side after WASM loads.

**Impact**:
- Additional network latency
- API rate limiting issues
- Compounded delay after WASM initialization

## Implemented Optimizations

### 1. Cargo.toml Release Profile
```toml
[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Enable Link Time Optimization
codegen-units = 1   # Better optimization
panic = 'abort'     # Reduce binary size
strip = true        # Strip symbols
```

###  2. Dioxus.toml WASM Optimizations
```toml
[web.wasm-opt]
level = 'z'  # Optimize for size

[web.pre-compress]
enabled = true
```

### 3. Eliminated Duplicate Server Calls
- Fixed `Blogs` component to only call `list_files()` once
- Used `use_memo` to derive article path from articles list
- Resources now properly depend on each other

## Recommendations for Further Optimization

### Immediate Actions:

1. **Use Release Mode for Testing**
   ```bash
   dx serve --release --addr 0.0.0.0 --port 8080
   ```
   This will significantly reduce WASM size and load time.

2. **Install wasm-opt** (if not already installed)
   ```bash
   cargo install wasm-opt
   ```

3. **Monitor WASM Bundle Size**
   ```bash
   ls -lh target/dx/blogger/release/web/public/assets/*.wasm
   ```
   Goal: < 500KB after compression

### Medium-Term Improvements:

4. **Add Loading Skeletons**
   - Show placeholder content while data loads
   - Improves perceived performance
   - Better user experience

5. **Implement Progressive Loading**
   - Load critical content first
   - Lazy-load GitHub repos
   - Use Suspense boundaries

6. **Consider Server-Side Rendering (SSR)**
   - Pre-render initial HTML on server
   - Hydrate with WASM when ready
   - Content visible before WASM loads

7. **Code Splitting**
   - Split large dependencies into separate chunks
   - Load markdown renderer on-demand
   - Lazy-load icon library

### Long-Term Optimizations:

8. **Caching Strategy**
   - Cache GitHub API responses
   - Use service workers
   - Implement stale-while-revalidate

9. **CDN for Assets**
   - Host WASM and CSS on CDN
   - Reduce server load
   - Improve global latency

10. **Alternative Rendering Strategies**
    - Static Site Generation (SSG) for articles
    - Incremental Static Regeneration
    - Edge rendering

## Expected Performance Improvements

| Optimization | Dev Mode | Release Mode |
|-------------|----------|--------------|
| Current | ~53s to first request | ~53s |
| With release build | ~53s | **~2-5s** |
| With SSR | ~53s | **< 1s** |
| With code splitting | **~20-30s** | **< 1s** |

## Development vs Production

**Important**: The 53-second delay is primarily a **development mode** issue:
- Dev mode includes debug symbols
- No optimizations applied
- Source maps generated
- Hot reload overhead

**Production builds** with the implemented optimizations should see:
- 90%+ reduction in WASM size
- 95%+ reduction in load time
- First Contentful Paint < 2 seconds

## Testing Performance

### Development Mode:
```bash
dx serve --addr 0.0.0.0 --port 8080
```

### Release Mode (Recommended for performance testing):
```bash
dx serve --release --addr 0.0.0.0 --port 8080
```

### Production Build:
```bash
dx build --release
```

## Monitoring

Check server logs for timing:
```
[INFO] [200] /                                    # Page load
[INFO] [200] /api/list_files...                  # First server function
```

Calculate: `first_server_function_time - page_load_time = WASM_init_time`

Target: < 2 seconds in release mode

## Next Steps

1. Test with `dx serve --release` to verify optimizations
2. Add loading indicators for better UX
3. Consider implementing SSR if < 2s is not acceptable
4. Profile WASM bundle to identify large dependencies
5. Evaluate GitHub API caching strategy
