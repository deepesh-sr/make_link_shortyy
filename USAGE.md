# Link Shortener - Usage Guide

## 🚀 Features Implemented

Your link shortener now has the following features:

1. **Shorten URLs** - Create short links for long URLs
2. **Custom Short Codes** - Use custom codes or auto-generate them
3. **Click Tracking** - Track how many times each link is clicked
4. **Redirect** - Automatically redirect to original URLs
5. **Health Check** - Monitor server status
6. **Prometheus Metrics** - Track server performance

---

## 📡 API Endpoints

### 1. Health Check
```bash
GET /health
```

**Example:**
```bash
curl http://localhost:3000/health
```

**Response:**
```
Service is healthy
```

---

### 2. Create Shortened Link
```bash
POST /shorten
Content-Type: application/json
```

**Request Body:**
```json
{
  "url": "https://www.example.com/very/long/url",
  "custom_code": "optional-custom-code"
}
```

**Example with Auto-Generated Code:**
```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.google.com"}'
```

**Example with Custom Code:**
```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.github.com", "custom_code": "github"}'
```

**Success Response:**
```json
{
  "short_code": "abc123",
  "short_url": "http://localhost:3000/abc123",
  "original_url": "https://www.google.com"
}
```

**Error Responses:**
- `400 Bad Request` - Invalid URL or custom code
- `409 Conflict` - Custom code already taken
- `500 Internal Server Error` - Database error

---

### 3. Redirect to Original URL
```bash
GET /{short_code}
```

**Example:**
```bash
curl -L http://localhost:3000/abc123
```

This will redirect you to the original URL and increment the click count.

**Error Responses:**
- `404 Not Found` - Short code doesn't exist
- `500 Internal Server Error` - Database error

---

### 4. View Metrics
```bash
GET /metrics
```

**Example:**
```bash
curl http://localhost:3000/metrics
```

Returns Prometheus-formatted metrics for monitoring.

---

## 🏃 Running the Server

### 1. Make sure your `.env` file is set up:
```bash
DATABASE_URL=postgresql://neondb_owner:npg_0IO9PTvfAHjE@ep-jolly-moon-ahlwv1gs-pooler.c-3.us-east-1.aws.neon.tech/neondb?sslmode=require&channel_binding=require
```

### 2. Run the migrations (if not already done):
```bash
sqlx migrate run
```

### 3. Start the server:
```bash
cargo run
```

You should see:
```
🚀 Link shortener server listening on http://0.0.0.0:3000
📊 Metrics available at http://0.0.0.0:3000/metrics
❤️  Health check at http://0.0.0.0:3000/health
```

---

## 🧪 Testing Examples

### Test 1: Create a shortened link
```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.rust-lang.org"}'
```

### Test 2: Use the shortened link
```bash
# Copy the short_code from the previous response (e.g., "Xy9K2p")
curl -L http://localhost:3000/Xy9K2p
```

### Test 3: Create a custom short code
```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.youtube.com", "custom_code": "yt"}'
```

### Test 4: Use the custom code
```bash
curl -L http://localhost:3000/yt
```

### Test 5: Try to reuse a custom code (should fail)
```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.example.com", "custom_code": "yt"}'
```

Expected error:
```json
{
  "error": "This custom code is already taken"
}
```

---

## 📊 Database Schema

The `links` table structure:
```sql
CREATE TABLE links (
    id BIGSERIAL PRIMARY KEY,
    short_code VARCHAR(10) UNIQUE NOT NULL,
    original_url TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    click_count INTEGER DEFAULT 0
);

CREATE INDEX idx_short_code ON links(short_code);
```

---

## 🔧 Validation Rules

### URL Validation:
- Must start with `http://` or `https://`
- Cannot be empty

### Custom Code Validation:
- Length: 3-10 characters
- Only alphanumeric characters allowed
- Must be unique (not already taken)

### Auto-Generated Codes:
- Length: 6 characters (or 8 if collision occurs)
- Uses alphanumeric characters
- Automatically checks for uniqueness

---

## 🎯 Next Steps

You can extend this link shortener with:

1. **Analytics Dashboard** - Use the `get_all_links()` and `get_stats()` functions
2. **Link Expiration** - Add expiry dates to links
3. **User Authentication** - Associate links with user accounts
4. **Rate Limiting** - Prevent abuse
5. **Custom Domains** - Support custom short domains
6. **QR Codes** - Generate QR codes for shortened links

---

## 🐛 Troubleshooting

### Server won't start:
- Check that your `DATABASE_URL` in `.env` is correct
- Make sure migrations are run: `sqlx migrate run`
- Check if port 3000 is available

### Database connection fails:
- Verify your Neon database credentials
- Check internet connection (Neon is cloud-hosted)
- Ensure TLS is enabled in your connection string

### "Custom code already taken" error:
- Choose a different custom code
- Query the database to see existing codes:
  ```sql
  SELECT short_code FROM links;
  ```

---

## 📝 Code Structure

```
src/
├── main.rs       # Server setup and routing
├── routes.rs     # HTTP handlers (shorten_link, redirect_to_url)
├── crud.rs       # Database operations (create_link, get_link_by_code, etc.)
└── .env          # Database connection string
```

---

Enjoy your link shortener! 🎉
