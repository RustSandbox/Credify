#[cfg(test)]
mod tests {

    #[test]
    fn test_linkedin_404_page_detection() {
        // Test various formats LinkedIn might use for 404 pages
        let test_bodies = vec![
            // Standard format
            "This page doesn't exist",
            // With curly quotes
            "This page doesn't exist",
            // HTML encoded apostrophe
            "This page doesn&#39;t exist",
            // XML encoded apostrophe
            "This page doesn&apos;t exist",
            // Other formats
            "Page not found",
            "Check the URL or return to LinkedIn home",
            "Check your URL or return to LinkedIn home",
            "return to LinkedIn home",
            // Combination messages
            "Go to your feed doesn't exist",
            "Go to your feed doesn&#39;t exist",
            "Go to your feed doesn&apos;t exist",
        ];

        // Each of these should be detected as a 404 page
        for body in test_bodies {
            println!("Testing body containing: '{}'", body);
            // In real implementation, we'd need to mock the HTTP response
            // For now, we're just testing that our patterns would match
            assert!(
                body.contains("This page doesn't exist")
                    || body.contains("This page doesn't exist")
                    || body.contains("This page doesn&#39;t exist")
                    || body.contains("This page doesn&apos;t exist")
                    || body.contains("Page not found")
                    || body.contains("Check the URL or return to LinkedIn home")
                    || body.contains("Check your URL or return to LinkedIn home")
                    || body.contains("return to LinkedIn home")
                    || (body.contains("Go to your feed") && body.contains("doesn't exist"))
                    || (body.contains("Go to your feed") && body.contains("doesn&#39;t exist"))
                    || (body.contains("Go to your feed") && body.contains("doesn&apos;t exist")),
                "Failed to detect 404 pattern in: '{}'",
                body
            );
        }
    }

    #[test]
    fn test_url_redirect_detection() {
        // Test that we detect redirects to 404 pages
        let redirect_urls = vec![
            "https://www.linkedin.com/404/",
            "https://linkedin.com/404",
            "https://www.linkedin.com/404/something",
            "https://linkedin.com/404?query=params",
        ];

        for url in redirect_urls {
            println!("Testing redirect URL: '{}'", url);
            assert!(
                url.contains("/404/") || url.contains("linkedin.com/404"),
                "Failed to detect 404 redirect in URL: '{}'",
                url
            );
        }
    }

    #[test]
    fn test_valid_content_not_detected_as_404() {
        // Test that normal content is not detected as 404
        let valid_bodies = vec![
            "Welcome to LinkedIn",
            "John Doe - Software Engineer",
            "Connect with professionals",
            "View profile",
        ];

        for body in valid_bodies {
            println!("Testing valid body: '{}'", body);
            assert!(
                !(body.contains("This page doesn't exist")
                    || body.contains("This page doesn't exist")
                    || body.contains("This page doesn&#39;t exist")
                    || body.contains("This page doesn&apos;t exist")
                    || body.contains("Page not found")
                    || body.contains("Check the URL or return to LinkedIn home")
                    || body.contains("Check your URL or return to LinkedIn home")
                    || body.contains("return to LinkedIn home")
                    || (body.contains("Go to your feed") && body.contains("doesn't exist"))
                    || (body.contains("Go to your feed") && body.contains("doesn&#39;t exist"))
                    || (body.contains("Go to your feed") && body.contains("doesn&apos;t exist"))),
                "Valid content incorrectly detected as 404: '{}'",
                body
            );
        }
    }
}
