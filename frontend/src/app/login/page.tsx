import { login } from "@/app/actions/auth";
import { redirect } from "next/navigation";
import { cookies } from "next/headers";
import { createHash } from "crypto";

function computeExpectedValue(): string {
    const password = process.env.ANALYZER_PASSWORD;
    const secret = process.env.AUTH_SECRET;
    if (!password || !secret) return "";
    return createHash("sha256")
        .update(password + ":" + secret)
        .digest("hex");
}

export default async function LoginPage({
    searchParams,
}: {
    searchParams: Promise<{ error?: string; redirect?: string }>;
}) {
    // If already authenticated, redirect away
    const cookieStore = await cookies();
    const sessionCookie = cookieStore.get("session")?.value;
    const expected = computeExpectedValue();
    const { error, redirect: redirectTo } = await searchParams;

    if (sessionCookie && sessionCookie === expected) {
        redirect(redirectTo || "/");
    }

    return (
        <div className="flex min-h-screen items-center justify-center">
            <div className="w-full max-w-sm space-y-6 p-6">
                <div className="text-center space-y-2">
                    <h1 className="text-2xl font-bold">
                        osu! beatmap analyzer
                    </h1>
                    <p className="text-sm text-muted-foreground">
                        Enter the shared password to access this tool.
                    </p>
                </div>

                <form action={login} className="space-y-4">
                    <input
                        type="hidden"
                        name="redirect"
                        value={redirectTo || "/"}
                    />

                    {error && (
                        <div className="rounded bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">
                            {error}
                        </div>
                    )}

                    <div className="space-y-2">
                        <label
                            htmlFor="password"
                            className="text-sm font-medium"
                        >
                            Password
                        </label>
                        <input
                            id="password"
                            type="password"
                            name="password"
                            required
                            className="w-full rounded border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                        />
                    </div>

                    <button
                        type="submit"
                        className="w-full rounded bg-primary px-3 py-2 text-sm font-semibold text-primary-foreground hover:bg-primary/90"
                    >
                        Sign in
                    </button>
                </form>
            </div>
        </div>
    );
}
