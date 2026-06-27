/** @type {import('next').NextConfig} */
const nextConfig = {
    eslint: {
        ignoreDuringBuilds: true,
    },
    images: {
        remotePatterns: [{ hostname: "assets.ppy.sh" }],
    },
};

export default nextConfig;
