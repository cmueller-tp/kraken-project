import React from "react";

type FlameIconProps = {};

export default function FlameIcon(props: FlameIconProps) {
    return (
        <div className={"icon"} {...props}>
            <svg
                className="neon"
                fill="none"
                stroke="#000"
                strokeLinecap="round"
                strokeLinejoin="round"
                width="800px"
                height="800px"
                strokeWidth="32px"
                viewBox="0 0 512 512"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path d="M112,320c0-93,124-165,96-272,66,0,192,96,192,272a144,144,0,0,1-288,0Z" />
                <path d="M320,368c0,57.71-32,80-64,80s-64-22.29-64-80,40-86,32-128C266,240,320,310.29,320,368Z" />
            </svg>
        </div>
    );
}
