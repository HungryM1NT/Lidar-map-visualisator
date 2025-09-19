import open3d as o3d

def main():
    pcd = o3d.io.read_point_cloud("../assets/processed_points.pcd")
    print(pcd)
    # o3d.visualization.draw_geometries([pcd])
    o3d.io.write_point_cloud("hak_ascii.pcd", pcd, write_ascii=True, compressed=False)
    # print("Hello from python-pcd!")


if __name__ == "__main__":
    main()
