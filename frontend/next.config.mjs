/** @type {import('next').NextConfig} */
const nextConfig = {
    eslint: {
        ignoreDuringBuilds: true,
    },
    experimental: {
        serverActions: {
            allowedOrigins: [
                "127.0.0.1:3006",
                process.env.VERCEL_URL && `*.${process.env.VERCEL_URL}`,
                process.env.VERCEL_URL,
            ].filter(Boolean),
        },
    },
    images: {
        remotePatterns: [{ hostname: "assets.ppy.sh" }],
    },
};

export default nextConfig;
