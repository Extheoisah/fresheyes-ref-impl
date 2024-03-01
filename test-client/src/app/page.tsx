"use client";
import React, { useEffect, useState, useMemo } from 'react';
import { GitHubServiceClient,} from '@/fresheyes_pb_service';
import { ForkRequest, ForkResult, Empty } from "@/fresheyes_pb";

export default function Home() {
    const [forkResult, setForkResult] = useState<ForkResult | null>(null);

    const client = useMemo(() => new GitHubServiceClient('http://localhost:50051'), []);
    console.log('client:', client);

    const checkHealth = () => {
        const empty = new Empty();
        client.check(empty, (error, result) => {
            if (error) {
                console.error('Error checking health:', error);
            } else {
                console.log('Health:', result?.toObject());
            }
        });
    }

    useEffect(() => {
        const forkRequest = new ForkRequest();
        forkRequest.setOwner('bitcoin');
        forkRequest.setRepo('bitcoin');

        client.forkRepository(forkRequest, (error, result) => {
            if (error) {
                console.error('Error forking repository:', error);
            } else {
                console.log('Fork Result:', result?.toObject());
                setForkResult(result)
            }
        });
    }, [client]);

    return (
        <main className="flex min-h-screen flex-col items-center justify-between p-24">
            <h1 className="text-6xl font-bold">Welcome to FreshEyes</h1>
            <p className="text-2xl">
                FreshEyes is a tool for automating the process of creating pull requests on GitHub.
            </p>

            <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onClick={checkHealth}>
                Check Health
            </button>
            {forkResult && (
                <div>
                    <h2>Fork Result:</h2>
                    <p>{JSON.stringify(forkResult.toObject())}</p>
                </div>
            )}
        </main>
    );
}
