# Cobotium Website and Documentation

This directory contains the website and documentation for the Cobotium project.

## Structure

- `docs/`: Documentation website files
  - `index.html`: Main documentation page
  - `deployment.html`: Mainnet deployment guide
  - `monitoring.html`: Monitoring setup guide
  - `styles.css`: Styling for documentation pages
- `vercel.json`: Configuration for Vercel deployment

## Deployment

The documentation is configured to be deployed on Vercel. To deploy:

1. Make sure you have the Vercel CLI installed:
   ```bash
   npm install -g vercel
   ```

2. Login to Vercel:
   ```bash
   vercel login
   ```

3. Deploy the website:
   ```bash
   vercel
   ```

4. To deploy to production:
   ```bash
   vercel --prod
   ```

## Customization

### Adding New Pages

To add a new documentation page:

1. Create a new HTML file in the `docs/` directory
2. Follow the same structure as the existing pages
3. Add a link to the new page in the sidebar of `index.html`

### Updating Content

To update existing documentation:

1. Edit the relevant HTML file
2. Deploy the changes using Vercel

### Adding Images

To add images:

1. Place the image files in the `docs/images/` directory
2. Reference them in your HTML using:
   ```html
   <img src="images/your-image.png" alt="Description">
   ```

## Integration with cobotium.io

The documentation is designed to be integrated with the main Cobotium website at cobotium.io. You can:

1. Link to the documentation from your main website
2. Embed the documentation in an iframe
3. Use the same styling across both sites for consistency

## Local Development

To test the documentation locally:

1. Install a simple HTTP server:
   ```bash
   npm install -g http-server
   ```

2. Run the server from the website directory:
   ```bash
   http-server
   ```

3. Open your browser to http://localhost:8080/docs/
